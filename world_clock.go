// world_clock.go - Мировые часы на Go (веб-сервер)
package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"time"
)

var citiesMap = map[string]float64{
	"Лондон": 0, "Париж": 1, "Москва": 3, "Дубай": 4,
	"Нью-Йорк": -5, "Лос-Анджелес": -8, "Токио": 9,
	"Пекин": 8, "Сидней": 11, "Нью-Дели": 5.5,
	"Сан-Паулу": -3, "Йоханнесбург": 2, "Сингапур": 8,
	"Гонконг": 8, "Стамбул": 3, "Рим": 1, "Берлин": 1,
	"Киев": 3, "Минск": 3, "Алматы": 6, "Новосибирск": 7,
	"Владивосток": 10, "Магадан": 11, "Камчатка": 12,
	"Лиссабон": 0, "Дублин": 0, "Афины": 2, "Каир": 2,
	"Найроби": 3, "Кейптаун": 2, "Бангкок": 7, "Джакарта": 7,
	"Сеул": 9, "Окленд": 13, "Гонолулу": -10, "Анкоридж": -9,
	"Чикаго": -6, "Денвер": -7, "Финикс": -7, "Майами": -5,
	"Торонто": -5, "Ванкувер": -8, "Мехико": -6, "Буэнос-Айрес": -3,
	"Рио-де-Жанейро": -3, "Перт": 8, "Аделаида": 10.5, "Канберра": 11,
}

const dataFile = "world_cities.json"

func loadCities() []string {
	if data, err := ioutil.ReadFile(dataFile); err == nil {
		var cities []string
		if err := json.Unmarshal(data, &cities); err == nil {
			return cities
		}
	}
	return []string{"Москва", "Лондон", "Нью-Йорк", "Токио", "Сидней"}
}

func saveCities(cities []string) {
	data, _ := json.MarshalIndent(cities, "", "  ")
	ioutil.WriteFile(dataFile, data, 0644)
}

func main() {
	cities := loadCities()
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		html := `<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>🌍 Мировые часы - Go</title>
<style>*{box-sizing:border-box;margin:0;}body{background:#1e2a3a;font-family:'Segoe UI',system-ui;display:flex;justify-content:center;min-height:100vh;padding:20px;}
.container{max-width:800px;width:100%;background:#2c3e50;border-radius:16px;padding:20px;box-shadow:0 10px 30px rgba(0,0,0,0.3);}
.header{display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:10px;margin-bottom:20px;color:white;}
.header h1{font-size:24px;}
.controls{display:flex;gap:8px;flex-wrap:wrap;}
.controls button,.controls select{padding:6px 12px;border:none;border-radius:6px;cursor:pointer;background:#3498db;color:white;}
.controls button:hover{background:#2980b9;}
.controls .danger{background:#e74c3c;}
.controls .danger:hover{background:#c0392b;}
.clock-list{display:flex;flex-direction:column;gap:8px;max-height:600px;overflow-y:auto;padding-right:5px;}
.clock-item{background:#34495e;border-radius:12px;padding:12px 16px;display:flex;align-items:center;flex-wrap:wrap;gap:12px;color:white;}
.clock-item:hover{background:#3d566e;}
.city-name{font-size:16px;font-weight:600;min-width:120px;}
.time-display{font-size:28px;font-family:'Courier New',monospace;font-weight:bold;min-width:130px;color:#2ecc71;}
.date-display{font-size:13px;color:#bdc3c7;min-width:140px;}
.delete-btn{margin-left:auto;background:#e74c3c;border:none;color:white;width:28px;height:28px;border-radius:50%;cursor:pointer;font-size:16px;line-height:1;}
.delete-btn:hover{background:#c0392b;}
.empty-msg{color:#7f8c8d;text-align:center;padding:40px;font-size:18px;}
::-webkit-scrollbar{width:6px;}
::-webkit-scrollbar-track{background:#2c3e50;border-radius:8px;}
::-webkit-scrollbar-thumb{background:#5d6d7e;border-radius:8px;}
@media (max-width:600px){.time-display{font-size:22px;min-width:100px;}.city-name{min-width:80px;font-size:14px;}}
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
const CITIES = ` + fmt.Sprintf("%v", citiesMap) + `;
let cities = ` + fmt.Sprintf("%v", cities) + `;
let format24 = true;

function saveCities(){ fetch('/save',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(cities)}); }

function updateClock(){
const now=new Date(); const utc=now.getTime()+now.getTimezoneOffset()*60000;
const list=document.getElementById('clockList'); list.innerHTML='';
if(cities.length===0){ list.innerHTML='<div class="empty-msg">🌍 Нет городов. Добавьте первый!</div>'; return; }
cities.forEach(city=>{
const offset=CITIES[city]; if(offset===undefined)return;
const localTime=new Date(utc+offset*3600000);
let hours=localTime.getHours(); const minutes=String(localTime.getMinutes()).padStart(2,'0'); const seconds=String(localTime.getSeconds()).padStart(2,'0');
let timeStr;
if(format24){ timeStr=` + "`${String(hours).padStart(2,'0')}:${minutes}:${seconds}`" + `; }
else{ const ampm=hours>=12?'PM':'AM'; hours=hours%12||12; timeStr=` + "`${String(hours).padStart(2,'0')}:${minutes}:${seconds} ${ampm}`" + `; }
const days=['Вс','Пн','Вт','Ср','Чт','Пт','Сб']; const months=['янв','фев','мар','апр','мая','июн','июл','авг','сен','окт','ноя','дек'];
const dateStr=` + "`${days[localTime.getDay()]}, ${localTime.getDate()} ${months[localTime.getMonth()]} ${localTime.getFullYear()}`" + `;
const item=document.createElement('div'); item.className='clock-item';
item.innerHTML=` + "`<span class=\"city-name\">${city}</span><span class=\"time-display\">${timeStr}</span><span class=\"date-display\">${dateStr}</span><button class=\"delete-btn\" data-city=\"${city}\">✕</button>`" + `;
list.appendChild(item);
});
document.querySelectorAll('.delete-btn').forEach(btn=>{
btn.addEventListener('click',function(){ const city=this.dataset.city; cities=cities.filter(c=>c!==city); saveCities(); updateClock(); updateSelect(); });
});
}

function updateSelect(){ const sel=document.getElementById('citySelect'); const current=sel.value; sel.innerHTML='<option value="">-- Добавить город --</option>';
Object.keys(CITIES).filter(c=>!cities.includes(c)).sort().forEach(c=>{ const opt=document.createElement('option'); opt.value=c; opt.textContent=c; sel.appendChild(opt); });
if(current && Object.keys(CITIES).includes(current) && !cities.includes(current)) sel.value=current;
}

document.getElementById('addBtn').onclick=()=>{ const sel=document.getElementById('citySelect'); const city=sel.value; if(!city)return; if(!cities.includes(city)){ cities.push(city); saveCities(); updateClock(); updateSelect(); } sel.value=''; };
document.getElementById('formatBtn').onclick=()=>{ format24=!format24; updateClock(); };
document.getElementById('clearBtn').onclick=()=>{ if(cities.length===0)return; if(confirm('Удалить все города?')){ cities=[]; saveCities(); updateClock(); updateSelect(); } };
updateSelect(); updateClock(); setInterval(updateClock,1000);
</script>
</body></html>`
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		fmt.Fprint(w, html)
	})

	http.HandleFunc("/save", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		body, _ := ioutil.ReadAll(r.Body)
		var cities []string
		if err := json.Unmarshal(body, &cities); err == nil {
			saveCities(cities)
			fmt.Fprint(w, "ok")
		} else {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
		}
	})

	log.Println("Сервер запущен на http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
