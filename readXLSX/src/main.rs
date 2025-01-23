use calamine::{Reader,open_workbook,Xlsx,Data};




fn main(){
    println!("Hello, world!");
    read_xlsx();
}

fn read_xlsx(){
     let mut workbook:Xlsx<_> = open_workbook("test_data.xlsx").expect("Failed to open workbook");
     let sheets = workbook.sheet_names().to_owned();
     println!("Sheets: {:?}",sheets);

     if let Ok(range) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!("Found {} cells in 'Sheet1', including {} non empty cells",
                 total_cells, non_empty_cells);
        // alternatively, we can manually filter rows
        assert_eq!(non_empty_cells, range.rows()
            .flat_map(|r| r.iter().filter(|&c| c != &Data::Empty)).count());

        //print all the rows
    for row in range.rows(){
        println!("{:?}",row);
    }
    }

    

     return;
}
