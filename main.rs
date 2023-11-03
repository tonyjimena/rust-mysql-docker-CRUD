use warp::{Filter, Rejection, Reply};
use mysql::*;
use mysql::prelude::*;
use mysql::{Pool, OptsBuilder}; // Importa las dependencias relacionadas con MySQL
use dotenv::dotenv;
use std::env;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Task {
    description: String,
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let create_task = warp::path!("tasks" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .map(create_task);

    let read_tasks = warp::path!("tasks" / "read")
        .and(warp::get())
        .map(read_tasks);

    let update_task = warp::path!("tasks" / "update" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .map(update_task);

    let delete_task = warp::path!("tasks" / "delete" / i32)
        .and(warp::delete())
        .map(delete_task);

    let routes = create_task.or(read_tasks).or(update_task).or(delete_task);

    warp::serve(routes)
        .run(([0, 0, 0,0], 3030))
        .await;
}

fn create_task(body: Task) -> impl Reply {
    let description = body.description;
    let url = "mysql://travellist_user:password@travellist-db:/travellist";
    let opts = OptsBuilder::from_opts(url);
    let pool = Pool::new(opts).expect("No se puede conectar a MySQL");

    let mut conn = pool.get_conn().expect("No se puede obtener una conexión");
    conn.exec_drop(
        r"INSERT INTO tasks (description) VALUES (:description)",
        params! {
            "description" => description,
        },
    ).expect("No se puede insertar tarea en MySQL");

    warp::reply::json(&"Tarea creada")
}

fn read_tasks() -> impl Reply {
    let url = "mysql://travellist_user:password@travellist-db:/travellist";
    let opts = OptsBuilder::from_opts(url);
    let pool = Pool::new(opts).expect("No se puede conectar a MySQL");

    let mut conn = pool.get_conn().expect("No se puede obtener una conexión");
    let result = conn.query_map::<(i32, String), _, _, Task>(
        r"SELECT id, description FROM tasks",
        |(id, description)| {
            Task { description }
        },
    ).expect("No se puede leer tareas de MySQL");

    warp::reply::json(&result)
}

fn update_task(id: i32, body: Task) -> impl Reply {
    let description = body.description;
    let url = "mysql://travellist_user:password@travellist-db:/travellist";
    let opts = OptsBuilder::from_opts(url);
    let pool = Pool::new(opts).expect("No se puede conectar a MySQL");

    let mut conn = pool.get_conn().expect("No se puede obtener una conexión");
    conn.exec_drop(
        r"UPDATE tasks SET description = :description WHERE id = :id",
        params! {
            "description" => description,
            "id" => id,
        },
    ).expect("No se puede actualizar tarea en MySQL");

    warp::reply::json(&"Tarea actualizada")
}

fn delete_task(id: i32) -> impl Reply {
    let url = "mysql://travellist_user:password@travellist-db:/travellist";
    let opts = OptsBuilder::from_opts(url);
    let pool = Pool::new(opts).expect("No se puede conectar a MySQL");

    let mut conn = pool.get_conn().expect("No se puede obtener una conexión");
    conn.exec_drop(
        r"DELETE FROM tasks WHERE id = :id",
        params! {
            "id" => id,
        },
    ).expect("No se puede eliminar tarea en MySQL");

    warp::reply::json(&"Tarea eliminada")
}
