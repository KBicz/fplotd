use std::fs;
use std::process::exit;

pub fn range(start: i32, stop: i32, step: usize) -> Vec<f64>
{
    let mut result: Vec<f64> = vec![0.0; (stop-start) as usize];
    for i in (start..stop).step_by(step) { result[i as usize] = i as f64; }
    return result;
}

pub fn rangef64(start: f64, stop: f64, step: f64) -> Vec<f64>
{
    let nel: usize = ((stop-start)/step) as usize;
    let mut result: Vec<f64> = vec![start; nel];
    for i in 0..nel { result[i] += (i as f64) *step; }
    return result;
}

pub fn read_data(path: &str, xc: usize, yc: usize, errxc: usize, erryc: usize, errxctrl: bool, erryctrl: bool, com: char, header: usize, deli: char, nth: usize) -> (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>)
{
    let data = fs::read_to_string(path).expect("# Unable to read file!!");
    let mut datavec: Vec<&str> = data.lines().collect();
    datavec = datavec.into_iter().filter(|&i| i.trim() != "" && i.chars().next().unwrap() != com).collect::<Vec<_>>();
    let (mut x, mut y, mut ex, mut ey): (Vec<f64>, Vec<f64>,Vec<f64>, Vec<f64>) = (vec![],vec![],vec![],vec![]);

    let mut line: Vec<&str> = datavec[0].split(deli).collect();
    line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
    let ncols = line.len();
    if xc > ncols || yc > ncols {exit(1);}
    if ncols == 0 {exit(1);}
    else if ncols == 1
    {
        x = range(0,datavec.len() as i32, nth); 
        for i in (header..datavec.len()).step_by(nth)
        {
            line = datavec[i].split(deli).collect();
            line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
            y.push(line[0].parse().unwrap());
            ex.push(0.0); ey.push(0.0);
        }
    }
    else
    {
        for i in (header..datavec.len()).step_by(nth)
        {
            line = datavec[i].split(" ").collect();
            line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
            x.push(line[xc].parse().unwrap()); y.push(line[yc].parse().unwrap());
            if errxctrl {ex.push(line[errxc].parse().unwrap());}
            else {ex.push(0.0);}
            if erryctrl {ey.push(line[erryc].parse().unwrap());}
            else {ey.push(0.0);}
        }
    }

    return (x,y,ex,ey);
}