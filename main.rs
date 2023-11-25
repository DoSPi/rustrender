use std::fs;
#[derive(Copy)]
#[derive(Clone)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8
}

struct Screen{
     pixels: [Pixel;200*200],
}

impl Screen{
    fn  get(&self, x : usize, y : usize)-> &Pixel {
        &self.pixels[200*y + x]
    }
    fn set(&mut self, x :usize, y :usize, color : &Pixel){
        let t : &mut Pixel = &mut self.pixels[200*y + x];
        *t = color.clone();
    }
    fn save_ppm(&self){
        let mut buf = String::from("P3\n200 200\n255\n");
        for y in (0..200).rev() {
            for x in 0..200 {
                let &p = self.get(x,y);
                let newline = if x == 200 -1 {"\n"} else {" "}; 
                buf = buf + &p.r.to_string() + " "+  &p.g.to_string() +" "+ &p.b.to_string() + newline;
            }
        }
        fs::write("output.ppm",buf).expect("Unable to write file");
    }

}

fn draw_line(screen : &mut Screen ,x0 : usize, y0 : usize, x1 : usize, y1 : usize){
    let mut y = y0;
    let dx = x1 as i64 - x0 as i64;
    let dy = y1 as i64 - y0 as i64;
    let mut d : i64 = 2 * dy - dx;
    let color = Pixel{r : 255, g : 255, b : 255};
    for x in x0..(x1 + 1) {
        screen.set(x,y, &color);
        if d > 0 {
            y += 1;
            d -= 2 * dx;
        }
        d  +=  2*dy;
    }
}

fn main(){
    let mut screen = Screen{pixels : [Pixel{r:0, g:0, b:0};200*200]};
    draw_line(&mut screen,0,0,100,100);
    screen.save_ppm();
}
