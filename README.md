# fplotd
fplotd - rust program to plot data and create scientific plot ready for publication from given file, code uses gnuplot crate created by SiegeLord (https://github.com/SiegeLord/RustGnuplot). Compile it using command "cargo build --release".

To run the code properly you have to install gnuplot.

    Program fplotd for Linux, MacOS, and Windows written by K. Bicz, ver. of Aug 1, 2021.
    Plot data from given file.

    Usage: fplotd <[-f=]str(s)> [-xc=i64] [-yc=i64] [-xerr=i64] [-yerr=i64] [-xmin=f64] [-xmax=f64]
                  [-ymin=f64] [-ymax=f64] [-xlab=str] [-ylab=str] [-tit=str] [-lsty=i64] [-syms=char]
                  [-mars=f64] [-xwid=i64] [-ywid=i64] [-xpos=f64] [-ypos=f64] [-thk=f64] [-legx=f64]
                  [-legy=f64] [-com=char] [-head=i64] [-deli=str] [-nth=i64] [-xw=f32] [-yw=f32]
                  [-fonts=i64] [-fontx=i64] [-fonty=i64] [-fontt=i64] [-phase=f64] [-hline=f64] [-vline=f64]
                  [--save[=str]] [--leg] [--xlog]  [--ylog] [--xln] [--yln] [--min] [--max] [--pal] [--qt]
                  [--line]

         option  -f     : input file (for multiple files div. between files is ",".)
                 -xc    : x column to plot (def. 1, for multiple files div. between columns is ",").
                 -yc    : y column to plot (def. 2, for multiple files div. between columns is ",").
                 -yerr  : column to plot y errors.
                 -xerr  : column to plot x errors.
                 -xmin  : min x value.
                 -xmax  : max x value.
                 -ymin  : min y value.
                 -ymax  : max y value.
                 -xlab  : x axis title.
                 -ylab  : y axis title.
                 -tit   : plot title name.
                 -lsty  : line style (def. Solid, SmallDot, Dot, Dash, DotDash, DotDotDash).
                 -syms  : symbol style (def. "O").
                 -mars  : symbol size (def. 0.5).
                 -xwid  : x width of window while displaying plot (default 1000 px).
                 -ywid  : y height of window while displaying plot (default 600 px).
                 -xpos  : x position of plot window (variates from 0 to 1, def. 0).
                 -ypos  : y position of plot window (variates from 0 to 1, def. 0).
                 -thk   : curve thickness (def. 2).
                 -legx  : legend location in x (def. 1).
                 -legy  : legend location in y (def. 1).
                 -com   : comment symbol (def. "#").
                 -head  : number of lines used as a header if not commented (default head = 0).
                 -deli  : delimiter symbol (def. " ").
                 -nth   : read every nth data point (def. nth = 1).
                 -xw    : x scale width of window while saving to  pdf\eps (default 10 times normal).
                 -yw    : y scale height of window while saving to pdf\eps (default 5 times normal).
                 -fonts : size of font on x & y axis.
                 -fontx : size of font of x axis.
                 -fonty : size of font of y axis.
                 -fontt : size of font of title.
                 -phase : phase data using given period (default period = 1.0).
                 -hline : plot horizontal line (you can divide lines using comma ",").
                 -vline : plot vertical line (you can divide lines using comma ",").
                 --save : savename to save plot (possible formats: pdf, eps, png, svg).
                 --leg  : show legend.
                 --xlog : x axis to base 10 logarithm.
                 --ylog : y axis to base 10 logarithm.
                 --xln  : x axis to base e logarithm.
                 --yln  : y axis to base e logarithm.
                 --min  : point minimal value (only with --leg).
                 --max  : point maximal value (only with --leg).
                 --pal  : lines and symbols.
                 --qt   : set terminal to qt (only usable in linux).
                 --line : plot line.
