#[allow(dead_code)]
pub(crate) async fn get_mssql_effected_rows(client: &dyn welds::Client) -> u32 {
	let rows = client.fetch_rows("SELECT @@ROWCOUNT", &[]).await.unwrap();
	let row = rows.first().unwrap();
	return row.get_by_position::<i32>(0).unwrap() as u32;
}
