mod loge;
mod log10;
mod plotd;
mod readdata;
mod checkformat;

use std::env;
use std::process::exit;

fn helpf() 
{
    println!("\n  Program fplotd for Linux, MacOS, and Windows written by K. Bicz, ver. of Aug 1, 2021.");
    println!("  Plot data from given file.");
    println!("\n  Usage: fplotd <[-f=]str(s)> [-xc=i64] [-yc=i64] [-xerr=i64] [-yerr=i64] [-xmin=f64] [-xmax=f64]");
    println!("         [-ymin=f64] [-ymax=f64] [-xlab=str] [-ylab=str] [-tit=str] [-lsty=i64] [-syms=char]");
    println!("         [-mars=f64] [-xwid=i64] [-ywid=i64] [-xpos=f64] [-ypos=f64] [-thk=f64] [-legx=f64]"); 
    println!("         [-legy=f64] [-com=char] [-head=i64] [-deli=str] [-nth=i64] [-xw=f32] [-yw=f32]");
    println!("         [-fonts=i64] [-fontx=i64] [-fonty=i64] [-fontt=i64] [-phase=f64] [-hline=f64] [-vline=f64]");
    println!("         [--save[=str]] [--leg] [--xlog]  [--ylog] [--xln] [--yln] [--min] [--max] [--pal] [--qt]");
    println!("         [--line]\n");
    println!("         option  -f     : input file (for multiple files div. between files is \",\".)");
    println!("                 -xc    : x column to plot (def. 1, for multiple files div. between columns is \",\").");
    println!("                 -yc    : y column to plot (def. 2, for multiple files div. between columns is \",\").");
    println!("                 -yerr  : column to plot y errors.");
    println!("                 -xerr  : column to plot x errors.");
    println!("                 -xmin  : min x value.");
    println!("                 -xmax  : max x value.");
    println!("                 -ymin  : min y value."); 
    println!("                 -ymax  : max y value.");
    println!("                 -xlab  : x axis title.");
    println!("                 -ylab  : y axis title.");
    println!("                 -tit   : plot title name.");
    println!("                 -lsty  : line style (def. Solid, SmallDot, Dot, Dash, DotDash, DotDotDash).");
    println!("                 -syms  : symbol style (def. \"O\").");
    println!("                 -mars  : symbol size (def. 0.5).");
    println!("                 -xwid  : x width of window while displaying plot (default 1000 px).");  
    println!("                 -ywid  : y height of window while displaying plot (default 600 px).");
    println!("                 -xpos  : x position of plot window (variates from 0 to 1, def. 0).");
    println!("                 -ypos  : y position of plot window (variates from 0 to 1, def. 0).");
    println!("                 -thk   : curve thickness (def. 2).");
    println!("                 -legx  : legend location in x (def. 1).");
    println!("                 -legy  : legend location in y (def. 1).");
    println!("                 -com   : comment symbol (def. \"#\").");
    println!("                 -head  : number of lines used as a header if not commented (default head = 0).");
    println!("                 -deli  : delimiter symbol (def. \" \")."); 
    println!("                 -nth   : read every nth data point (def. nth = 1).");
    println!("                 -xw    : x scale width of window while saving to  pdf\\eps (default 10 times normal).");
    println!("                 -yw    : y scale height of window while saving to pdf\\eps (default 5 times normal).");  
    println!("                 -fonts : size of font on x & y axis.");  
    println!("                 -fontx : size of font of x axis.");  
    println!("                 -fonty : size of font of y axis.");  
    println!("                 -fontt : size of font of title.");  
    println!("                 -phase : phase data using given period (default period = 1.0).");
    println!("                 -hline : plot horizontal line (you can divide lines using comma \",\").");    
    println!("                 -vline : plot vertical line (you can divide lines using comma \",\").");
    println!("                 --save : savename to save plot (possible formats: pdf, eps, png, svg).");
    println!("                 --leg  : show legend.");
    println!("                 --xlog : x axis to base 10 logarithm.");
    println!("                 --ylog : y axis to base 10 logarithm.");  
    println!("                 --xln  : x axis to base e logarithm.");
    println!("                 --yln  : y axis to base e logarithm.");  
    println!("                 --min  : point minimal value (only with --leg).");
    println!("                 --max  : point maximal value (only with --leg).");
    println!("                 --pal  : lines and symbols.");
    println!("                 --qt   : set terminal to qt (only usable in Linux).");
    println!("                 --line : plot line.\n");
    exit(0);
}

fn main() -> Result<(), &'static str> 
{

    let mut v: Vec<&str>;
    let mut qt: bool = false;
    let mut noinputctrl: bool = true;
    let argc: usize = env::args().len();
    let mut legendstr: Vec<&str> = vec![];
    let (mut helpx, mut yloc): (usize, usize);
    let mut sformat: String = "pdf".to_string();
    let argv: Vec<String> = env::args().collect();
    let (mut com, mut deli): (char, char) = ('#', ' ');
    let mut result: (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>);
    let (ymino, ymaxo, xmino, xmaxo): (f64, f64, f64, f64);
    let (mut xmins, mut xmaxs): (Vec<f64>, Vec<f64>) = (vec![], vec![]);
    let (mut ymins, mut ymaxs): (Vec<f64>, Vec<f64>) = (vec![], vec![]);
    let (mut xw, mut yw, mut fonts): (f32, f32, f64) = (10f32, 5f32, 15.0f64);
    let (mut files, mut savename) : (Vec<&str>, &str) = (vec!["input.txt"], "plot.pdf");  
    let (mut fontx, mut fonty, mut fontt): (f64, f64, f64) = (15.0f64, 15.0f64, 13.0f64);
    let (mut xc, mut yc, mut nth): (Vec<&str>, Vec<&str>, usize) = (vec!["0"],vec!["1"],1);
    let (mut xmin, mut xmax, mut ymin, mut ymax): (f64, f64, f64, f64) = (0f64, 0f64, 0f64, 0f64);
    let (mut xwid, mut ywid, mut xpos, mut ypos): (f32, f32, f64, f64) = (1000f32,600f32,0f64,0f64);
    let (mut xminc, mut xmaxc, mut yminc, mut ymaxc): (bool, bool, bool, bool) = (false, false, false, false);
    let (mut lsty, mut syms, mut mars, mut thk): (String, char, f64, f64) = ("Solid".to_string(),'O',0.75,2.0);
    let (mut xlabel, mut ylabel, mut tit): (String, String, String) = ("X []".to_string(),"Y []".to_string(),"".to_string());
    let (mut legx, mut legy, mut header, mut period, mut phasectrl): (f64, f64, usize, f64, bool) = (0.99, 0.99, 0, 1.0, false);
    let (mut errxctrl, mut erryctrl, mut errxc, mut erryc): (bool, bool, usize, usize) = (false, false, 3 as usize, 4 as usize);
    let (mut log10x, mut log10y, mut lnx, mut lny, mut linectrl): (bool, bool, bool, bool, bool) = (false, false, false, false,false);
    let (mut minctrl, mut maxctrl, mut legctrl, mut pal, mut savectrl): (bool, bool, bool, bool, bool) = (false, false, false, false, false);
    let (mut xdata, mut ydata, mut errx, mut erry): (Vec<Vec<f64>>,Vec<Vec<f64>>,Vec<Vec<f64>>,Vec<Vec<f64>>) = (vec![],vec![],vec![],vec![]);
    let (mut hline, mut vline, mut hlinevecx, mut hlinevecy, mut vlinevecx, mut vlinevecy): (Vec<&str>, Vec<&str>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) = (vec![""], vec![""], vec![], vec![], vec![], vec![]);

    if argc < 2 {helpf();}
    else if argc == 2 && !argv[1].eq("--help") && !argv[1].eq("-h") { files[0] = &argv[1]; }
    else 
    {
        for i in 0..argc
        {
            if argv[i].contains("-f=") {v = argv[i].split("=").collect(); files[0] = v[1]; noinputctrl = false;}
            else if argv[i].contains("-xc=") {v = argv[i].split("=").collect(); xc[0] = v[1];} 
            else if argv[i].contains("-yc=") {v = argv[i].split("=").collect(); yc[0] = v[1];} 
            else if argv[i].contains("-xerr=") 
            {v= argv[i].split("=").collect(); errxc = v[1].parse().unwrap(); errxctrl = true;}
            else if argv[i].contains("-yerr=") 
            {v= argv[i].split("=").collect(); erryc = v[1].parse().unwrap(); erryctrl = true;}
            else if argv[i].contains("-xmin=") {v = argv[i].split("=").collect(); xmin = v[1].parse().unwrap(); xminc = true;}
            else if argv[i].contains("-xmax=") {v = argv[i].split("=").collect(); xmax = v[1].parse().unwrap(); xmaxc = true;}
            else if argv[i].contains("-ymin=") {v = argv[i].split("=").collect(); ymin = v[1].parse().unwrap(); yminc = true; }
            else if argv[i].contains("-ymax=") {v = argv[i].split("=").collect(); ymax = v[1].parse().unwrap(); ymaxc = true;}
            else if argv[i].contains("-xlab=") {v = argv[i].split("=").collect(); xlabel = v[1].parse().unwrap();}
            else if argv[i].contains("-ylab=") {v = argv[i].split("=").collect(); ylabel = v[1].parse().unwrap();}
            else if argv[i].contains("-tit=") {v = argv[i].split("=").collect(); tit = v[1].parse().unwrap();}
            else if argv[i].contains("-lsty") {v = argv[i].split("=").collect(); lsty = v[1].parse().unwrap();}    
            else if argv[i].contains("-syms") {v = argv[i].split("=").collect(); syms = v[1].parse().unwrap();}    
            else if argv[i].contains("-mars") {v = argv[i].split("=").collect(); mars = v[1].parse().unwrap();}    
            else if argv[i].contains("-thk") {v = argv[i].split("=").collect(); thk = v[1].parse().unwrap();}     
            else if argv[i].contains("-legx=") {v = argv[i].split("=").collect(); legx = v[1].parse().unwrap();}
            else if argv[i].contains("-legy=") {v = argv[i].split("=").collect(); legy = v[1].parse().unwrap();}
            else if argv[i].contains("-com=") {v = argv[i].split("=").collect(); com = v[1].parse().unwrap();}
            else if argv[i].contains("-head=") {v = argv[i].split("=").collect(); header = v[1].parse().unwrap();}
            else if argv[i].contains("-deli=") {v = argv[i].split("=").collect(); deli = v[1].parse().unwrap();}
            else if argv[i].contains("-nth=") {v = argv[i].split("=").collect(); nth = v[1].parse().unwrap();}
            else if argv[i].contains("-xw=") {v = argv[i].split("=").collect(); xw = v[1].parse().unwrap();}
            else if argv[i].contains("-yw=") {v = argv[i].split("=").collect(); yw = v[1].parse().unwrap();}
            else if argv[i].contains("-xwid") {v = argv[i].split("=").collect(); xwid = v[1].parse().unwrap();}
            else if argv[i].contains("-ywid") {v = argv[i].split("=").collect(); ywid = v[1].parse().unwrap();}
            else if argv[i].contains("-xpos=") {v = argv[i].split("=").collect(); xpos = v[1].parse().unwrap();}
            else if argv[i].contains("-ypos=") {v = argv[i].split("=").collect(); ypos = v[1].parse().unwrap();}
            else if argv[i].contains("-fonts=") {v = argv[i].split("=").collect(); fonts = v[1].parse().unwrap();}
            else if argv[i].contains("-fontx=") {v = argv[i].split("=").collect(); fontx = v[1].parse().unwrap();}
            else if argv[i].contains("-fonty=") {v = argv[i].split("=").collect(); fonty = v[1].parse().unwrap();}
            else if argv[i].contains("-fontt=") {v = argv[i].split("=").collect(); fontt = v[1].parse().unwrap();}
            else if argv[i].contains("-phase=") {v = argv[i].split("=").collect(); period = v[1].parse().unwrap(); phasectrl = true;}
            else if argv[i].contains("-hline=") {v = argv[i].split("=").collect(); hline[0] = v[1];}
            else if argv[i].contains("-vline=") {v = argv[i].split("=").collect(); vline[0] = v[1];}
            else if argv[i].contains("--save") 
            { savectrl = true; if argv[i].contains("=") {v = argv[i].split("=").collect(); savename = v[1];} }
            else if argv[i].eq("--leg") {legctrl = true;}   
            else if argv[i].eq("--xlog") {log10x = true;}
            else if argv[i].eq("--ylog") {log10y = true;}
            else if argv[i].eq("--xln") {lnx = true;}
            else if argv[i].eq("--yln") {lny = true;}
            else if argv[i].eq("--min") {minctrl = true;} 
            else if argv[i].eq("--max") {maxctrl = true;}
            else if argv[i].eq("--pal") {pal = true;}
            else if argv[i].eq("--qt") {qt = true;}
            else if argv[i].eq("--line") {linectrl = true;}
            else if argv[i].eq("--help") || argv[i].eq("-h")  {helpf();}
        }
    }
    
    if xpos < 0.0 {xpos = xpos.abs();} if ypos < 0.0 {ypos = ypos.abs();}
    if xpos > 1.0 {xpos -= xpos.floor();} if ypos > 1.0 {ypos -= ypos.floor();}
    if noinputctrl {files[0] = &argv[1];}
    if erryc > 0 {erryc -= 1;} if errxc > 0 {errxc -= 1;}
    if savectrl && !savename.eq("plot.pdf") { sformat = checkformat::checkformat(savename.to_string()); }
    
    if files[0].contains(",") { files = files[0].split(",").collect(); }
    if hline[0].contains(",") { hline = hline[0].split(",").collect();}
    if vline[0].contains(",") { vline = vline[0].split(",").collect();}
    if xc[0].contains(",") { xc = xc[0].split(",").collect(); }
    else { for _ in 1..files.len() {xc.push(xc[0]);}}
    if yc[0].contains(",") { yc = yc[0].split(",").collect(); }
    else { for _ in 1..files.len() {yc.push(yc[0]);}}

    if xc.len() != yc.len() || files.len() != xc.len() || yc.len() != files.len() 
    { println!("# Error! Your number of columns differ from number of given files!"); exit(0);}

    for i in 0..files.len()
    {
        if std::path::Path::new(files[i]).exists() 
        {
            legendstr.push(files[i]);
            result = readdata::read_data(files[i], xc[i].parse().unwrap(), yc[i].parse().unwrap(), errxc, erryc, errxctrl, erryctrl, com, header, deli, nth);
            xdata.push(result.0);
            ydata.push(result.1);
            errx.push(result.2);
            erry.push(result.3);

            if phasectrl { for j in 0..xdata[i].len() { xdata[i][j] = xdata[i][j]/period%1.0; } }
        }

        if lnx { xdata[i] = loge::ln(&xdata[i]).to_vec(); }
        if lny { ydata[i] = loge::ln(&ydata[i]).to_vec(); }
        if log10x { xdata[i] = log10::log10(&xdata[i]).to_vec(); }
        if log10y { ydata[i] = log10::log10(&ydata[i]).to_vec(); }

        xmins.push(xdata[i].clone().iter().fold(f64::INFINITY, |a, &b| a.min(b)));
        xmaxs.push(xdata[i].clone().iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)));
        ymins.push(ydata[i].clone().iter().fold(f64::INFINITY, |a, &b| a.min(b)));
        ymaxs.push(ydata[i].clone().iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)));
    }

    ymino = ymins.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    yloc = ymins.iter().position(|&r| r == ymino).unwrap(); 
    let yminloc = ydata[yloc].clone().iter().position(|&r| r == ymino).unwrap();
    xmino = xdata[yloc][yminloc];
    if !yminc { ymin = ymino.clone(); if erryctrl {ymin -= erry[yloc][yminloc];} }

    ymaxo = ymaxs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    yloc = ymaxs.iter().position(|&r| r == ymaxo).unwrap();
    let ymaxloc = ydata[yloc].iter().position(|&r| r == ymaxo).unwrap();
    xmaxo = xdata[yloc][ymaxloc];
    if !ymaxc { ymax = ymaxo.clone(); ymin -= (ymax-ymin)*0.05; }

    if !xminc { xmin = xmins.iter().fold(f64::INFINITY, |a, &b| a.min(b)); }
    if !xmaxc { xmax = xmaxs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)); xmax += (xmax-xmin)*0.05; }
    if !xminc { xmin -= (xmax-xmin)*0.05; }
    if !yminc { ymax += (ymax-ymin)*0.05; }

    if erryctrl 
    {
        ymax += erry[ymins.iter().position(|&r| r == ymino).unwrap()][ymaxloc];
        ymin -= erry[yloc][yminloc];
    } 
    if errxctrl 
    {
        helpx = xmaxs.iter().position(|&r| r == xmax).unwrap();
        xmax += errx[helpx][xdata[helpx].clone().iter().position(|&r| r == xmax).unwrap()];
        helpx = xmins.iter().position(|&r| r == xmin).unwrap();
        xmin -= errx[helpx][xdata[helpx].clone().iter().position(|&r| r == xmin).unwrap()];
    } 

    if !vline[0].eq("") 
    {
        for i in 0..vline.len()
        {
            vlinevecy.push(readdata::rangef64(ymin,ymax,(ymax-ymin)/1000.0));
            vlinevecx.push(vec![vline[i].parse().unwrap(); vlinevecy[i].len()]);
            
        }
    }
    if !hline[0].eq("") 
    {
        for i in 0..hline.len()
        {
            hlinevecx.push(readdata::rangef64(xmin,xmax,(xmax-xmin)/1000.0));
            hlinevecy.push(vec![hline[i].parse().unwrap(); hlinevecx[i].len()]);
        }
    }

    plotd::fplot(xdata,ydata,errx,erry,xmin,xmax,ymin,ymax,xw,yw,fonts,fontx,fonty,fontt,xwid,ywid,xpos,ypos,xlabel,ylabel,tit,lsty,syms,mars,thk,legx,legy,legctrl,minctrl,maxctrl,xmino,xmaxo,ymino,ymaxo,pal,linectrl,errxctrl,erryctrl,legendstr,savectrl,sformat,savename,qt,hlinevecx,hlinevecy,vlinevecx,vlinevecy)
}