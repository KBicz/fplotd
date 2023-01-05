use std::fs;
use std::process::exit;

pub fn range(start: i32, stop: i32) -> Vec<f64>
{
    let mut result: Vec<f64> = vec![0.0; (stop-start) as usize];
    for i in start..stop { result[i as usize] = i as f64; }
    return result;
}

pub fn read_data(path: &str, xc: usize, yc: usize, errxc: usize, erryc: usize, errxctrl: bool, erryctrl: bool, com: char, header: usize, deli: char) -> (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>)
{
    let data: String = fs::read_to_string(path).expect("# Unable to read file!!");
    let mut datavec: Vec<&str> = data.lines().collect();
    datavec = datavec.into_iter().filter(|&i| i.trim() != "" && i.chars().next().unwrap() != com).collect::<Vec<_>>();
    let (mut x, mut y, mut ex, mut ey): (Vec<f64>, Vec<f64>,Vec<f64>, Vec<f64>) = (vec![0.0; datavec.len()],vec![0.0; datavec.len()],vec![0.0; datavec.len()],vec![0.0; datavec.len()]);
    
    if header > datavec.len() { println!("# Error! Header can not be longer than the data file!"); exit(0); }

    let mut line: Vec<&str> = datavec[0].split(deli).collect();
    line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
    let ncols: usize = line.len();
    if xc > ncols || yc > ncols {exit(1);}
    if ncols == 0 {exit(1);}
    else if ncols == 1
    {
        x = range(0,datavec.len() as i32); 
        for i in header..datavec.len() 
        {
            line = datavec[i].split(deli).collect();
            line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
            y[i] = line[0].parse().unwrap();
            ex[i] = 0.0; ey[i] = 0.0;
        }
    }
    else
    {
        for i in header..datavec.len() 
        {
            line = datavec[i].split(" ").collect();
            line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
            x[i] = line[xc].parse().unwrap(); y[i] = line[yc].parse().unwrap();
            if errxctrl {ex[i] = line[errxc].parse().unwrap();}
            else {ex[i] = 0.0;}
            if erryctrl {ey[i] = line[erryc].parse().unwrap();}
            else {ey[i] = 0.0;}
        }
    }

    return (x,y,ex,ey);
}