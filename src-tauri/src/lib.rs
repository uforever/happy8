use sqlx::{prelude::FromRow, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager as _};

struct AppState {
    db: Pool<Sqlite>,
}

async fn setup_db(_app: &App) -> Pool<Sqlite> {
    // 指定的路径必须遵循与定义在
    // `tauri.conf.json > bundle > resources`
    // let resource_path = app
    //     .path()
    //     .resolve("resources/db.sqlite", BaseDirectory::Resource)
    //     .unwrap();

    SqlitePoolOptions::new()
        .connect("sqlite:resources/db.sqlite")
        .await
        .unwrap()
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct QueryResponse {
    total: usize,
    records: Vec<QueryResult>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct QueryResult {
    issue: String,
    winning: [u8; 20],
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TrendItem {
    issue: String,
    step: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, FromRow)]
struct QueryResultRow {
    id: i64,
    issue: String,
    has01: bool,
    has02: bool,
    has03: bool,
    has04: bool,
    has05: bool,
    has06: bool,
    has07: bool,
    has08: bool,
    has09: bool,
    has10: bool,
    has11: bool,
    has12: bool,
    has13: bool,
    has14: bool,
    has15: bool,
    has16: bool,
    has17: bool,
    has18: bool,
    has19: bool,
    has20: bool,
    has21: bool,
    has22: bool,
    has23: bool,
    has24: bool,
    has25: bool,
    has26: bool,
    has27: bool,
    has28: bool,
    has29: bool,
    has30: bool,
    has31: bool,
    has32: bool,
    has33: bool,
    has34: bool,
    has35: bool,
    has36: bool,
    has37: bool,
    has38: bool,
    has39: bool,
    has40: bool,
    has41: bool,
    has42: bool,
    has43: bool,
    has44: bool,
    has45: bool,
    has46: bool,
    has47: bool,
    has48: bool,
    has49: bool,
    has50: bool,
    has51: bool,
    has52: bool,
    has53: bool,
    has54: bool,
    has55: bool,
    has56: bool,
    has57: bool,
    has58: bool,
    has59: bool,
    has60: bool,
    has61: bool,
    has62: bool,
    has63: bool,
    has64: bool,
    has65: bool,
    has66: bool,
    has67: bool,
    has68: bool,
    has69: bool,
    has70: bool,
    has71: bool,
    has72: bool,
    has73: bool,
    has74: bool,
    has75: bool,
    has76: bool,
    has77: bool,
    has78: bool,
    has79: bool,
    has80: bool,
}

// impl QueryResult {
//     fn new(issue: &str, winning: [u8; 20]) -> Self {
//         Self {
//             issue: String::from(issue),
//             winning,
//         }
//     }
// }

impl QueryResultRow {
    fn has_number(&self, number: u8) -> bool {
        match number {
            1 => self.has01,
            2 => self.has02,
            3 => self.has03,
            4 => self.has04,
            5 => self.has05,
            6 => self.has06,
            7 => self.has07,
            8 => self.has08,
            9 => self.has09,
            10 => self.has10,
            11 => self.has11,
            12 => self.has12,
            13 => self.has13,
            14 => self.has14,
            15 => self.has15,
            16 => self.has16,
            17 => self.has17,
            18 => self.has18,
            19 => self.has19,
            20 => self.has20,
            21 => self.has21,
            22 => self.has22,
            23 => self.has23,
            24 => self.has24,
            25 => self.has25,
            26 => self.has26,
            27 => self.has27,
            28 => self.has28,
            29 => self.has29,
            30 => self.has30,
            31 => self.has31,
            32 => self.has32,
            33 => self.has33,
            34 => self.has34,
            35 => self.has35,
            36 => self.has36,
            37 => self.has37,
            38 => self.has38,
            39 => self.has39,
            40 => self.has40,
            41 => self.has41,
            42 => self.has42,
            43 => self.has43,
            44 => self.has44,
            45 => self.has45,
            46 => self.has46,
            47 => self.has47,
            48 => self.has48,
            49 => self.has49,
            50 => self.has50,
            51 => self.has51,
            52 => self.has52,
            53 => self.has53,
            54 => self.has54,
            55 => self.has55,
            56 => self.has56,
            57 => self.has57,
            58 => self.has58,
            59 => self.has59,
            60 => self.has60,
            61 => self.has61,
            62 => self.has62,
            63 => self.has63,
            64 => self.has64,
            65 => self.has65,
            66 => self.has66,
            67 => self.has67,
            68 => self.has68,
            69 => self.has69,
            70 => self.has70,
            71 => self.has71,
            72 => self.has72,
            73 => self.has73,
            74 => self.has74,
            75 => self.has75,
            76 => self.has76,
            77 => self.has77,
            78 => self.has78,
            79 => self.has79,
            80 => self.has80,
            _ => false,
        }
    }
}

impl From<QueryResultRow> for QueryResult {
    fn from(value: QueryResultRow) -> Self {
        let mut winning = [0; 20];
        let mut index = 0;
        for i in 1..=80u8 {
            if value.has_number(i) {
                winning[index] = i;
                if index == 19 {
                    break;
                } else {
                    index += 1;
                }
            }
        }
        Self {
            issue: value.issue,
            winning,
        }
    }
}

#[tauri::command]
async fn query(issue: String, page: usize, size: usize, state: tauri::State<'_, AppState>) -> Result<QueryResponse, String> {
    let rows: Vec<QueryResultRow> =
        sqlx::query_as::<_, QueryResultRow>("SELECT * FROM happy8 WHERE issue LIKE ? ORDER BY id DESC")
            .bind(format!("%{}%", issue))
            .fetch_all(&state.db)
            .await
            .map_err(|e| format!("数据库查询错误：{}", e))?;

    let result: Vec<QueryResult> = rows.into_iter().map(|row| row.into()).collect();
    let total = result.len();
    let start = (page - 1) * size;
    let end = start + size;
    let records = result.into_iter().skip(start).take(end - start).collect();
    Ok(QueryResponse { total, records })
}

#[tauri::command]
async fn add(item: QueryResult, state: tauri::State<'_, AppState>) -> Result<String, String> {
    if item.issue.is_empty() {
        return Err("issue is required".to_string());
    }
    let sql_state: String = format!(
        "INSERT INTO happy8 (issue, {}) VALUES (?, {})",
        item.winning
            .iter()
            .map(|i| format!("has{:02}", i))
            .collect::<Vec<String>>()
            .join(", "),
        (0..20).map(|_| "true").collect::<Vec<&str>>().join(", ")
    );
    sqlx::query(&sql_state)
        .bind(item.issue)
        .execute(&state.db)
        .await
        .map_err(|e| format!("数据库插入错误：{}", e))?;
    Ok("录入成功".to_string())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn trend(red: Vec<u8>, state: tauri::State<'_, AppState>) -> Result<Vec<TrendItem>, String> {
    let mut true_columns = vec![];
    for r in red {
        if !(1..=80).contains(&r) {
            return Err("存在非法号码".to_string());
        }
        true_columns.push(format!("has{:02} = TRUE", r));
    }

    let sql_state: String = format!("SELECT * FROM happy8 WHERE {}", true_columns.join(" AND "));

    let rows: Vec<QueryResultRow> = sqlx::query_as::<_, QueryResultRow>(&sql_state)
        .fetch_all(&state.db)
        .await
        .map_err(|e| format!("数据库查询错误：{}", e))?;

    if rows.is_empty() {
        return Err("未找到匹配的记录".to_string());
    }

    if rows.len() < 3 {
        return Err(format!("找到 {} 条记录，过少，不足以分析趋势", rows.len()));
    }

    // 遍历 相邻两条记录 记录issue 为 "issue1-issue2"
    // 记录 step 为 id2 - id1 - 1
    let mut trends: Vec<TrendItem> = vec![];

    for i in 0..rows.len() - 1 {
        let issue = format!("{}-{}", rows[i].issue, rows[i + 1].issue);
        let step = rows[i + 1].id - rows[i].id - 1;
        trends.push(TrendItem { issue, step });
    }

    Ok(trends)
}

#[tauri::command]
async fn advance(where_clause: String, state: tauri::State<'_, AppState>) -> Result<Vec<TrendItem>, String> {
    let sql_state: String = format!("SELECT * FROM happy8 WHERE {}", where_clause);

    let rows: Vec<QueryResultRow> = sqlx::query_as::<_, QueryResultRow>(&sql_state)
        .fetch_all(&state.db)
        .await
        .map_err(|e| format!("数据库查询错误：{}", e))?;

    if rows.is_empty() {
        return Err("未找到匹配的记录".to_string());
    }

    if rows.len() < 3 {
        return Err(format!("找到 {} 条记录，过少，不足以分析趋势", rows.len()));
    }

    // 遍历 相邻两条记录 记录issue 为 "issue1-issue2"
    // 记录 step 为 id2 - id1 - 1
    let mut trends: Vec<TrendItem> = vec![];

    for i in 0..rows.len() - 1 {
        let issue = format!("{}-{}", rows[i].issue, rows[i + 1].issue);
        let step = rows[i + 1].id - rows[i].id - 1;
        trends.push(TrendItem { issue, step });
    }

    Ok(trends)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;
                app.manage(AppState { db });
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![trend, query, add, advance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
