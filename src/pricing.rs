//! Local, source-traceable price observations and activity assemblies.
//! No retailer crawling, API keys, or unlicensed datasets are bundled.
use axum::{extract::{Multipart, Path, State}, http::StatusCode, Json};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;
use super::{AppState, ApiResult, err, db};

pub fn init(c: &Connection) -> ApiResult<()> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS price_observations (
        id TEXT PRIMARY KEY, item_key TEXT NOT NULL, description TEXT NOT NULL,
        component_type TEXT NOT NULL, unit TEXT NOT NULL, package_price REAL NOT NULL,
        package_quantity REAL NOT NULL, zip_code TEXT NOT NULL, supplier TEXT NOT NULL,
        sku TEXT NOT NULL, observed_at TEXT NOT NULL, source_url TEXT NOT NULL,
        rights_note TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
    CREATE INDEX IF NOT EXISTS idx_prices_lookup ON price_observations(item_key,unit,zip_code,observed_at);
    CREATE TABLE IF NOT EXISTS assemblies (
        id TEXT PRIMARY KEY, label TEXT NOT NULL, category TEXT NOT NULL, output_unit TEXT NOT NULL, notes TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS assembly_components (
        assembly_id TEXT NOT NULL, item_key TEXT NOT NULL, component_unit TEXT NOT NULL,
        consumption REAL NOT NULL, waste_percent REAL NOT NULL DEFAULT 0,
        PRIMARY KEY(assembly_id,item_key,component_unit),
        FOREIGN KEY(assembly_id) REFERENCES assemblies(id)
    );").map_err(err)?;
    if c.query_row("SELECT COUNT(*) FROM assemblies",[],|r|r.get::<_,i64>(0)).map_err(err)? == 0 {
        // Quantities are illustrations requiring review. The application ships with NO market prices.
        let recipes:[(&str,&str,&str,&str,&str,&[(&str,&str,f64,f64)]);3] = [
            ("drywall_repair","Replace and finish 1/2-inch drywall","Drywall","SF","Illustrative recipe: gross repair area; confirm labor productivity, joint finishing, texture, demolition and access separately.", &[
                ("drywall_half_in","SF",1.0,10.0),
                ("drywall_install_labor","HR",0.08,0.0),
                ("drywall_finish_labor","HR",0.12,0.0),
            ]),
            ("interior_paint","Prime and paint interior surface","Painting","SF","Illustrative recipe: no special preparation, masking or ceilings; verify coverage and coats.", &[
                ("interior_paint_material","GAL",0.007,8.0),
                ("painter_labor","HR",0.04,0.0),
            ]),
            ("laminate_floor","Replace laminate flooring","Flooring","SF","Illustrative installation recipe; removal, underlayment and trim are excluded and must be scoped separately.", &[
                ("laminate_flooring","SF",1.0,12.0),
                ("flooring_install_labor","HR",0.065,0.0),
            ]),
        ];
        for (id,label,category,unit,notes,parts) in recipes {
            c.execute("INSERT INTO assemblies(id,label,category,output_unit,notes) VALUES(?1,?2,?3,?4,?5)",params![id,label,category,unit,notes]).map_err(err)?;
            for &(key,punit,consumption,waste) in parts {
                c.execute("INSERT INTO assembly_components(assembly_id,item_key,component_unit,consumption,waste_percent) VALUES(?1,?2,?3,?4,?5)",params![id,key,punit,consumption,waste]).map_err(err)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)] struct IncomingRow {
    item_key: String, description: String, component_type: String, unit: String,
    package_price: f64, package_quantity: f64, zip_code: String, supplier: String,
    sku: String, observed_at: String, source_url: String, rights_note: String,
}
impl IncomingRow {
    fn validate(&self) -> Result<(), String> {
        if self.item_key.trim().is_empty() || self.item_key.len()>96 ||
           self.description.trim().is_empty() || self.description.len()>240 ||
           !["material","labor","equipment","disposal"].contains(&self.component_type.as_str()) ||
           !["SF","LF","EA","HR","GAL","SQ","CY","DAY","BDL"].contains(&self.unit.as_str()) ||
           !self.package_price.is_finite() || self.package_price < 0.0 || self.package_price>100_000_000.0 ||
           !self.package_quantity.is_finite() || self.package_quantity<=0.0 || self.package_quantity>1_000_000.0 ||
           !(self.zip_code=="*" || (self.zip_code.len()==5 && self.zip_code.bytes().all(|b|b.is_ascii_digit()))) ||
           self.supplier.trim().is_empty() || self.supplier.len()>120 ||
           self.sku.len()>120 || self.observed_at.len()!=10 ||
           !self.observed_at.bytes().enumerate().all(|(i,b)|if i==4||i==7 {b==b'-'} else {b.is_ascii_digit()}) ||
           self.source_url.len()>400 || !(self.source_url.is_empty()||self.source_url.starts_with("https://")||self.source_url.starts_with("http://")) ||
           self.rights_note.trim().is_empty() || self.rights_note.len()>200 {
            return Err("Invalid source observation. Verify fields, supported units, ISO date, rights_note and pricing.".into());
        }
        Ok(())
    }
}

pub async fn import_csv(State(s):State<AppState>,mut mp:Multipart)->ApiResult<Json<Value>> {
    let mut payload=None;
    while let Some(field)=mp.next_field().await.map_err(err)? {
        if field.name()==Some("file") {
            let bytes=field.bytes().await.map_err(err)?;
            if bytes.len()>5_000_000{return Err((StatusCode::PAYLOAD_TOO_LARGE,"CSV limit 5 MB".into()));}
            payload=Some(bytes);
            break;
        }
    }
    let bytes=payload.ok_or(err("Select a CSV file"))?;
    let mut rdr=csv::ReaderBuilder::new().trim(csv::Trim::All).from_reader(bytes.as_ref());
    let mut parsed=Vec::new();
    for (idx,row) in rdr.deserialize::<IncomingRow>().enumerate() {
        if idx>=5000{return Err(err("Maximum 5,000 observations per import"));}
        let r=row.map_err(|e|err(format!("CSV row {}: {}",idx+2,e)))?;
        r.validate().map_err(|e|err(format!("CSV row {}: {}",idx+2,e)))?;
        parsed.push(r);
    }
    if parsed.is_empty(){return Err(err("CSV has no observations"));}
    let mut c=db(&s)?;
    let tx=c.transaction().map_err(err)?;
    for r in &parsed {
        tx.execute("INSERT INTO price_observations(id,item_key,description,component_type,unit,package_price,package_quantity,zip_code,supplier,sku,observed_at,source_url,rights_note)
          VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",params![Uuid::new_v4().to_string(),r.item_key,r.description,r.component_type,r.unit,r.package_price,r.package_quantity,r.zip_code,r.supplier,r.sku,r.observed_at,r.source_url,r.rights_note]).map_err(err)?;
    }
    tx.commit().map_err(err)?;
    Ok(Json(json!({"imported":parsed.len(),"source":"User-supplied file; rights and accuracy not independently verified"})))
}

#[derive(Deserialize)] pub struct PricingQuery {pub zip:Option<String>,pub search:Option<String>}
pub async fn observations(State(s):State<AppState>,axum::extract::Query(q):axum::extract::Query<PricingQuery>)->ApiResult<Json<Value>> {
    let zip=q.zip.unwrap_or_default();
    if !zip.is_empty() && !(zip.len()==5 && zip.bytes().all(|b|b.is_ascii_digit())) {return Err(err("ZIP must contain five digits"));}
    let search=q.search.unwrap_or_default();
    let c=db(&s)?;
    let mut st=c.prepare("SELECT id,item_key,description,component_type,unit,package_price,package_quantity,
       ROUND(package_price/package_quantity,4) AS unit_cost,zip_code,supplier,sku,observed_at,source_url,rights_note
       FROM price_observations WHERE (?1='' OR zip_code=?1 OR zip_code='*')
       AND (?2='' OR item_key LIKE '%' || ?2 || '%' OR description LIKE '%' || ?2 || '%')
       ORDER BY observed_at DESC, supplier LIMIT 300").map_err(err)?;
    let names=(0..st.column_count()).map(|i|st.column_name(i).unwrap_or("").to_string()).collect::<Vec<_>>();
    let data=st.query_map(params![zip,search],|r|{
        let mut m=serde_json::Map::new();
        for (i,name) in names.iter().enumerate(){let v:rusqlite::types::Value=r.get(i)?;
          let j=match v {rusqlite::types::Value::Null=>Value::Null,rusqlite::types::Value::Integer(n)=>json!(n),rusqlite::types::Value::Real(n)=>json!(n),rusqlite::types::Value::Text(n)=>json!(n),rusqlite::types::Value::Blob(_)=>Value::Null};
          m.insert(name.clone(),j);
        } Ok(Value::Object(m))
    }).map_err(err)?.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?;
    Ok(Json(json!(data)))
}

pub async fn assemblies(State(s):State<AppState>)->ApiResult<Json<Value>> {
    let c=db(&s)?;
    let mut st=c.prepare("SELECT id,label,category,output_unit,notes FROM assemblies ORDER BY label").map_err(err)?;
    let rs=st.query_map([],|r|Ok(json!({"id":r.get::<_,String>(0)?,"label":r.get::<_,String>(1)?,"category":r.get::<_,String>(2)?,"output_unit":r.get::<_,String>(3)?,"notes":r.get::<_,String>(4)?}))).map_err(err)?;
    Ok(Json(json!(rs.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?)))
}

#[derive(Deserialize)] pub struct AssemblyRequest {pub assembly_id:String,pub zip_code:String,pub quantity:f64}
#[derive(Debug, Serialize)] pub struct CostComponent {
    pub item_key:String,pub unit:String,pub units_per_output:f64,pub unit_price:f64,pub cost_per_output:f64,
    pub supplier:String,pub observed_at:String,pub source_url:String,pub rights_note:String,pub selected_quote_id:String,
}
#[derive(Debug, Serialize)] pub struct Quote {pub assembly_id:String,pub label:String,pub category:String,pub output_unit:String,
    pub zip_code:String,pub quantity:f64,pub unit_price:f64,pub line_total:f64,pub components:Vec<CostComponent>}

// Strict exact-ZIP or explicit nationwide (*) quotes only. No geographic distance guessing.
// If more than one quote exists per key/locality, select the MEDIAN unit price from the newest
// observation per supplier+SKU, not the cheapest promotional listing.
pub fn quote(c:&Connection,input:&AssemblyRequest)->ApiResult<Quote> {
    if !(input.zip_code.len()==5 && input.zip_code.bytes().all(|b|b.is_ascii_digit())) ||
       !input.quantity.is_finite() || input.quantity<=0.0 || input.quantity>1_000_000.0 {return Err(err("Provide five-digit ZIP and a positive quantity"));}
    let (label,category,unit):(String,String,String)=c.query_row("SELECT label,category,output_unit FROM assemblies WHERE id=?1",params![input.assembly_id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).map_err(err)?;
    let mut st=c.prepare("SELECT item_key,component_unit,consumption,waste_percent FROM assembly_components WHERE assembly_id=?1 ORDER BY item_key").map_err(err)?;
    let recipe=st.query_map(params![input.assembly_id],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,f64>(2)?,r.get::<_,f64>(3)?))).map_err(err)?.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?;
    if recipe.is_empty(){return Err(err("Assembly has no components"));}
    let mut components=Vec::new();let mut missing=Vec::new();let mut sum=0.0;
    for (key,unit,consumption,waste) in recipe {
        let mut statement=c.prepare("SELECT id,package_price,package_quantity,zip_code,supplier,sku,observed_at,source_url,rights_note
            FROM price_observations WHERE item_key=?1 AND unit=?2 AND zip_code IN (?3,'*')
            ORDER BY CASE WHEN zip_code=?3 THEN 0 ELSE 1 END, observed_at DESC").map_err(err)?;
        let rows=statement.query_map(params![key,unit,input.zip_code],|r|Ok((r.get::<_,String>(0)?,r.get::<_,f64>(1)?,r.get::<_,f64>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,String>(5)?,r.get::<_,String>(6)?,r.get::<_,String>(7)?,r.get::<_,String>(8)?))).map_err(err)?;
        let raw=rows.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?;
        let has_local=raw.iter().any(|r|r.3==input.zip_code);
        let mut unique=HashMap::new();
        for r in raw.into_iter().filter(|r|!has_local||r.3==input.zip_code) {
            unique.entry((r.4.clone(),r.5.clone())).or_insert(r);
        }
        let mut observations=unique.into_values().collect::<Vec<_>>();
        if observations.is_empty(){missing.push(format!("{key} ({unit})"));continue;}
        observations.sort_by(|a,b|(a.1/a.2).total_cmp(&(b.1/b.2)));
        let selected=&observations[(observations.len()-1)/2];
        let price=selected.1/selected.2;
        let usage=consumption*(1.0+waste/100.0);
        let component_cost=price*usage;
        sum+=component_cost;
        components.push(CostComponent{item_key:key,unit,units_per_output:usage,unit_price:price,
             cost_per_output:component_cost,supplier:selected.4.clone(),observed_at:selected.6.clone(),
             source_url:selected.7.clone(),rights_note:selected.8.clone(),selected_quote_id:selected.0.clone()});
    }
    if !missing.is_empty(){return Err((StatusCode::UNPROCESSABLE_ENTITY,format!("Cannot price assembly: missing local/nationwide observations for {}. Import sourced quotes before estimating.",missing.join(", "))));}
    let unit_price=(sum*10000.0).round()/10000.0;
    let line_total=(input.quantity*unit_price*100.0).round()/100.0;
    Ok(Quote{assembly_id:input.assembly_id.clone(),label,category,output_unit:unit,zip_code:input.zip_code.clone(),quantity:input.quantity,unit_price,line_total,components})
}

pub async fn preview(Path(_):Path<String>,State(s):State<AppState>,Json(input):Json<AssemblyRequest>)->ApiResult<Json<Value>> {
    let c=db(&s)?;Ok(Json(json!(quote(&c,&input)?)))
}

pub async fn add_assembly_line(Path(rid):Path<String>,State(s):State<AppState>,Json(input):Json<AssemblyRequest>)->ApiResult<Json<Value>> {
    let mut c=db(&s)?;let result=quote(&c,&input)?;
    let source=serde_json::to_string(&result).map_err(err)?;
    let id=Uuid::new_v4().to_string();
    let tx=c.transaction().map_err(err)?;
    tx.execute("INSERT INTO lines(id,room_id,description,category,quantity,unit,unit_price,source) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",
        params![id,rid,result.label,result.category,result.quantity,result.output_unit,result.unit_price,format!("Quantimate price snapshot: {source}")]).map_err(err)?;
    tx.commit().map_err(err)?;
    Ok(Json(json!({"line_id":id,"quote":result,"notice":"Stored snapshot; future quote imports do not silently rewrite the estimate."})))
}

#[cfg(test)] mod tests {
  use super::*;
  #[test] fn costing_requires_all_components() {
    let c=Connection::open_in_memory().unwrap();init(&c).unwrap();
    let req=AssemblyRequest{assembly_id:"drywall_repair".into(),zip_code:"10001".into(),quantity:80.0};
    let e=quote(&c,&req).unwrap_err();
    assert_eq!(e.0,StatusCode::UNPROCESSABLE_ENTITY);
  }
  #[test] fn normalized_costing_uses_all_components() {
    let c=Connection::open_in_memory().unwrap();init(&c).unwrap();
    for (key,unit,price,pack) in [("drywall_half_in","SF",16.0,32.0),("drywall_install_labor","HR",40.0,1.0),("drywall_finish_labor","HR",50.0,1.0)] {
       c.execute("INSERT INTO price_observations(id,item_key,description,component_type,unit,package_price,package_quantity,zip_code,supplier,sku,observed_at,source_url,rights_note) VALUES(?1,?2,'synthetic','labor',?3,?4,?5,'10001','test','sku','2026-10-22','','test')",params![Uuid::new_v4().to_string(),key,unit,price,pack]).unwrap();
    }
    let result=quote(&c,&AssemblyRequest{assembly_id:"drywall_repair".into(),zip_code:"10001".into(),quantity:100.0}).unwrap();
    // 1.1 * ($16/32) + .08*$40 + .12*$50 = $9.75 / SF
    assert!((result.unit_price - 9.75).abs()<0.00001);
    assert!((result.line_total - 975.0).abs()<0.00001);
  }
  #[test] fn invalid_observation_rejected() {
      let r=IncomingRow{item_key:"demo".into(),description:"test".into(),component_type:"material".into(),unit:"SF".into(),package_price:f64::NAN,package_quantity:1.0,zip_code:"10001".into(),supplier:"test".into(),sku:"1".into(),observed_at:"2026-10-22".into(),source_url:"".into(),rights_note:"test".into()};
      assert!(r.validate().is_err());
  }
}
