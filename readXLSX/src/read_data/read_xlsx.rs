use calamine::{Reader,open_workbook,Xlsx,Data};
use std::collections;


pub fn read_data(){
    let mut workbook:Xlsx<_> = open_workbook("../test_data.xlsx").expect("Failed to open workbook");
    let sheets = workbook.sheet_names().to_owned();
    println!("Sheets: {:?}",sheets);

    if let Ok(range) = workbook.worksheet_range("Sheet1") {
       let total_cells = range.get_size().0 * range.get_size().1;
       let non_empty_cells: usize = range.used_cells().count();
       println!("Found {} cells in 'Sheet1', including {} non empty cells",
                total_cells, non_empty_cells);
       assert_eq!(non_empty_cells, range.rows()
           .flat_map(|r| r.iter().filter(|&c| c != &Data::Empty)).count());

       for row in range.rows() {
           let row_map: collections::HashMap<String, String> = row.iter()
               .map(|cell| cell.to_string())  // Convert Data enum to String
               .enumerate()  // Add index as key
               .map(|(i, v)| (i.to_string(), v))  // Convert index to string
               .collect();
           println!("{:?}", row_map);
       }
   }
    return;
}