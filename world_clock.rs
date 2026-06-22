// world_clock.rs - Мировые часы на Rust (веб-сервер)
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

type CityMap = HashMap<String, f64>;

lazy_static::lazy_static! {
    static ref CITIES: CityMap = {
        let mut m = HashMap::new();
        m.insert("Лондон".to_string(), 0.0);
        m.insert("Париж".to_string(), 1.0);
        m.insert("Москва".to_string(), 3.0);
        m.insert("Дубай".to_string(), 4.0);
        m.insert("Нью-Йорк".to_string(), -5.0);
        m.insert("Лос-Анджелес".to_string(), -8.0);
        m.insert("Токио".to_string(), 9.0);
        m.insert("Пекин".to_string(), 8.0);
        m.insert("Сидней".to_string(), 11.0);
        m.insert("Нью-Дели".to_string(), 5.5);
        m.insert("Сан-Паулу".to_string(), -3.0);
        m.insert("Йоханнесбург".to_string(), 2.0);
        m.insert("Сингапур".to_string(), 8.0);
        m.insert("Гонконг".to_string(), 8.0);
        m.insert("Стамбул".to_string(), 3.0);
        m.insert("Рим".to_string(), 1.0);
        m.insert("Берлин".to_string(), 1.0);
        m.insert("Киев".to_string(), 3.0);
        m.insert("Минск".to_string(), 3.0);
        m.insert("Алматы".to_string(), 6.0);
        m.insert("Новосибирск".to_string(), 7.0);
        m.insert("Владивосток".to_string(), 10.0);
        m.insert("Магадан".to_string(), 11.0);
        m.insert("Камчатка".to_string(), 12.0);
        m.insert("Лиссабон".to_string(), 0.0);
        m.insert("Дублин".to_string(), 0.0);
        m.insert("Афины".to_string(), 2.0);
        m.insert("Каир".to_string(), 2.0);
        m.insert("Найроби".to_string(), 3.0);
        m.insert("Кейптаун".to_string(), 2.0);
        m.insert("Бангкок".to_string(), 7.0);
        m.insert("Джакарта".to_string(), 7.0);
        m.insert("Сеул".to_string(), 9.0);
        m.insert("Окленд".to_string(), 13.0);
        m.insert("Гонолулу".to_string(), -10.0);
        m.insert("Анкоридж".to_string(), -9.0);
        m.insert("Чикаго".to_string(), -6.0);
        m.insert("Денвер".to_string(), -7.0);
        m.insert("Финикс".to_string(), -7.0);
        m.insert("Майами".to_string(), -5.0);
        m.insert("Торонто".to_string(), -5.0);
        m.insert("Ванкувер".to_string(), -8.0);
        m.insert("Мехико".to_string(), -6.0);
        m.insert("Буэнос-Айрес".to_string(), -3.0);
        m.insert("Рио-де-Жанейро".to_string(), -3.0);
        m.insert("Перт".to_string(), 8.0);
        m.insert("Аделаида".to_string(), 10.5);
        m.insert("Канберра".to_string(), 11.0);
        m
    };
}

const DATA_FILE: &str = "world_cities.json";

fn load_cities() -> Vec<String> {
    if let Ok(data) = fs::read_to_string(DATA_FILE) {
        if let Ok(cities) = serde_json::from_str(&data) {
            return cities;
        }
    }
    vec!["Москва".to_string(), "Лондон".to_string(), "Нью-Йорк".to_string(), "Токио".to_string(), "Сидней".to_string()]
}

fn save_cities(cities: &[String]) {
    if let Ok(data) = serde_json::to_string_pretty(cities) {
        let _ = fs::write(DATA_FILE, data);
    }
}

struct AppState {
    cities: Mutex<Vec<String>>,
}

async fn index(data: web::Data<AppState>) -> HttpResponse {
    let cities = data.cities.lock().unwrap();
    let cities_json = serde_json::to_string(&*cities).unwrap_or("[]".to_string());
    let cities_map = serde_json::to_string(&*CITIES).unwrap_or("{}".to_string());

    let html = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>🌍 Мировые часы - Rust</title>
<style>*{{box-sizing:border-box;margin:0;}}body{{background:#1e2a3a;font-family:'Segoe UI',system-ui;display:flex;justify-content:center;min-height:100vh;padding:20px;}}
.container{{max-width:800px;width:100%;background:#2c3e50;border-radius:16px;padding:20px;box-shadow:0 10px 30px rgba(0,0,0,0.3);}}
.header{{display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:10px;margin-bottom:20px;color:white;}}
.header h1{{font-size:24px;}}
.controls{{display:flex;gap:8px;flex-wrap:wrap;}}
.controls button,.controls select{{padding:6px 12px;border:none;border-radius:6px;cursor:pointer;background:#3498db;color:white;}}
.controls button:hover{{background:#2980b9;}}
.controls .danger{{background:#e74c3c;}}
.controls .danger:hover{{background:#c0392b;}}
.clock-list{{display:flex;flex-direction:column;gap:8px;max-height:600px;overflow-y:auto;padding-right:5px;}}
.clock-item{{background:#34495e;border-radius:12px;padding:12px 16px;display:flex;align-items:center;flex-wrap:wrap;gap:12px;color:white;}}
.clock-item:hover{{background:#3d566e;}}
.city-name{{font-size:16px;font-weight:600;min-width:120px;}}
.time-display{{font-size:28px;font-family:'Courier New',monospace;font-weight:bold;min-width:130px;color:#2ecc71;}}
.date-display{{font-size:13px;color:#bdc3c7;min-width:140px;}}
.delete-btn{{margin-left:auto;background:#e74c3c;border:none;color:white;width:28px;height:28px;border-radius:50%;cursor:pointer;font-size:16px;line-height:1;}}
.delete-btn:hover{{background:#c0392b;}}
.empty-msg{{color:#7f8c8d;text-align:center;padding:40px;font-size:18px;}}
::-webkit-scrollbar{{width:6px;}}
::-webkit-scrollbar-track{{background:#2c3e50;border-radius:8px;}}
::-webkit-scrollbar-thumb{{background:#5d6d7e;border-radius:8px;}}
@media (max-width:600px){{.time-display{{font-size:22px;min-width:100px;}}.city-name{{min-width:80px;font-size:14px;}}}}
</style></head>
<body>
<div class="container">
<div class="header"><h1>🌍 Мировые часы</h1>
<div class="controls">
<select id="citySelect"><option value="">-- Добавить город --</option></select>
<button id="addBtn">➕ Добавить</button>
<button id="formatBtn">12/24 ч</button>
<button id="clearBtn" class="danger">🗑️ Очистить</button>
</div></div>
<div class="clock-list" id="clockList"></div>
</div>
<script>
const CITIES = {};
const CITIES_OBJ = {};
let cities = {};
let format24 = true;

function saveCities(){{ fetch('/save',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify(cities) }}); }}

function updateClock(){{
const now=new Date(); const utc=now.getTime()+now.getTimezoneOffset()*60000;
const list=document.getElementById('clockList'); list.innerHTML='';
if(cities.length===0){{ list.innerHTML='<div class="empty-msg">🌍 Нет городов. Добавьте первый!</div>'; return; }}
cities.forEach(city=>{{
const offset=CITIES[city]; if(offset===undefined)return;
const localTime=new Date(utc+offset*3600000);
let hours=localTime.getHours(); const minutes=String(localTime.getMinutes()).padStart(2,'0'); const seconds=String(localTime.getSeconds()).padStart(2,'0');
let timeStr;
if(format24){{ timeStr=` + "`${String(hours).padStart(2,'0')}:${minutes}:${seconds}`" + `; }}
else{{ const ampm=hours>=12?'PM':'AM'; hours=hours%12||12; timeStr=` + "`${String(hours).padStart(2,'0')}:${minutes}:${seconds} ${ampm}`" + `; }}
const days=['Вс','Пн','Вт','Ср','Чт','Пт','Сб']; const months=['янв','фев','мар','апр','мая','июн','июл','авг','сен','окт','ноя','дек'];
const dateStr=` + "`${days[localTime.getDay()]}, ${localTime.getDate()} ${months[localTime.getMonth()]} ${localTime.getFullYear()}`" + `;
const item=document.createElement('div'); item.className='clock-item';
item.innerHTML=` + "`<span class=\"city-name\">${city}</span><span class=\"time-display\">${timeStr}</span><span class=\"date-display\">${dateStr}</span><button class=\"delete-btn\" data-city=\"${city}\">✕</button>`" + `;
list.appendChild(item);
}});
document.querySelectorAll('.delete-btn').forEach(btn=>{{
btn.addEventListener('click',function(){{ const city=this.dataset.city; cities=cities.filter(c=>c!==city); saveCities(); updateClock(); updateSelect(); }});
}});
}}

function updateSelect(){{ const sel=document.getElementById('citySelect'); const current=sel.value; sel.innerHTML='<option value="">-- Добавить город --</option>';
Object.keys(CITIES).filter(c=>!cities.includes(c)).sort().forEach(c=>{{ const opt=document.createElement('option'); opt.value=c; opt.textContent=c; sel.appendChild(opt); }});
if(current && Object.keys(CITIES).includes(current) && !cities.includes(current)) sel.value=current;
}}

document.getElementById('addBtn').onclick=()=>{{ const sel=document.getElementById('citySelect'); const city=sel.value; if(!city)return; if(!cities.includes(city)){{ cities.push(city); saveCities(); updateClock(); updateSelect(); }} sel.value=''; }};
document.getElementById('formatBtn').onclick=()=>{{ format24=!format24; updateClock(); }};
document.getElementById('clearBtn').onclick=()=>{{ if(cities.length===0)return; if(confirm('Удалить все города?')){{ cities=[]; saveCities(); updateClock(); updateSelect(); }} }};
updateSelect(); updateClock(); setInterval(updateClock,1000);
</script>
</body></html>"#,
        cities_json = cities_json,
        cities_map = cities_map,
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn save(data: web::Data<AppState>, body: web::Bytes) -> HttpResponse {
    if let Ok(cities) = serde_json::from_slice::<Vec<String>>(&body) {
        let mut state = data.cities.lock().unwrap();
        *state = cities;
        save_cities(&state);
        HttpResponse::Ok().body("ok")
    } else {
        HttpResponse::BadRequest().body("Invalid JSON")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_cities = load_cities();
    let app_state = web::Data::new(AppState {
        cities: Mutex::new(initial_cities),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/save", web::post().to(save))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
