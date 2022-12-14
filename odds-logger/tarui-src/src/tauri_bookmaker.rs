use data::{BookMaker, BookMakerBuilder, OddsError};
use odds::{EuropeOdds, OddsManager};
use tauri::State;

#[tauri::command]
pub async fn get_book_maker_lists(
    manager: State<'_, OddsManager>,
) -> Result<Vec<BookMaker>, OddsError> {
    let manager = &*manager;
    let bms = manager.list_bookermaker().await?;
    Ok(bms)
}

#[tauri::command]
pub async fn save_book_maker_info(
    manager: State<'_, OddsManager>,
    name: String,
    url: String,
    note: String,
) -> Result<Vec<BookMaker>, OddsError> {
    let manager = &*manager;
    let book_maker = BookMakerBuilder::default()
        .name(name)
        .url(url)
        .note(note)
        .build()
        .unwrap();
    let bms = manager.create_bookermaker(book_maker).await?;
    Ok(bms)
}

#[tauri::command]
pub async fn delete_book_maker_info(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<BookMaker>, OddsError> {
    let manager = &*manager;
    let bms = manager.delete_bookermaker(id).await?;
    Ok(bms)
}

#[tauri::command]
pub async fn get_book_maker_with_id(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<BookMaker, OddsError> {
    let manager = &*manager;
    let book_maker = manager.query_bookermaker_with_id(id).await?;
    Ok(book_maker)
}

#[tauri::command]
pub async fn update_book_maker(
    manager: State<'_, OddsManager>,
    id: i32,
    name: String,
    url: String,
    note: String,
) -> Result<Vec<BookMaker>, OddsError> {
    let manager = &*manager;
    let book_maker = BookMakerBuilder::default()
        .id(id)
        .name(name)
        .url(url)
        .note(note)
        .build()
        .unwrap();
    let bms = manager.update_bookermaker(book_maker).await?;
    Ok(bms)
}
