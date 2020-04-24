extern crate rand;
use std::io;
use rand::{Rand, Rng};
use std::io::Write;
use std::io::Read;

#[derive(Copy, Clone)]
struct Pixel {
    r: u8, g:u8, b:u8, a:u8
}
impl Pixel{
    fn merge(pix: Pixel, pix1: Pixel, pr: f32) -> Pixel{
        Pixel{r: pix.r + (pr * (pix1.r as f32 - pix.r as f32)) as u8,
            g: pix.g + (pr * (pix1.g as f32 - pix.g as f32)) as u8,
            b: pix.b + (pr * (pix1.b as f32 - pix.b as f32)) as u8,
            a :pix.a + (pr * (pix1.a as f32 - pix.a as f32)) as u8}
    }
}

fn merge_bytes(pix: &mut Pixel, bytes: [u8; 5], size: u8){
    if(size == 4){
        pix.r = bytes[2];
        pix.g = bytes[1];
        pix.b = bytes[0];
        pix.a = bytes[3];
    }
    else if(size == 3){
        pix.r = bytes[2];
        pix.g = bytes[1];
        pix.b = bytes[0];
        pix.a = 255;
    }
}


struct Pixel_set{
    w: u16,
    h: u16,
    data: Vec<Pixel>,
}

enum dir{
    Ox, Oy
}

trait pixel_fns{
    fn new(w: u16, h:u16) -> Pixel_set;
    fn set(pix: &mut Pixel_set, x: u16, y: u16, a: Pixel);
    fn draw_line(pix: &mut Pixel_set, x: f32, y: f32, x1:f32, y1: f32, a: Pixel);
    fn fill(pix: &mut Pixel_set, a: Pixel) -> ();
    fn draw_circle(pix: &mut Pixel_set, x: f32, y: f32, radius: f32, px: Pixel) -> ();
    fn draw_rect(pix: &mut Pixel_set, x1: u16, y1: u16, x2: u16, y2: u16, pixl: Pixel) -> ();
    fn draw_gradient(pix: &mut Pixel_set, px: Pixel, px1: Pixel, dr: dir) -> ();
}



impl pixel_fns for Pixel_set{
    fn new(wd: u16, hd:u16) -> Pixel_set{
        Pixel_set{w:wd, h:hd, data: Vec::with_capacity(wd as usize * hd as usize)}
}
    fn fill(pix: &mut Pixel_set, a: Pixel) -> (){
        for i in 0..(pix.w as usize * pix.h as usize){
            pix.data.push(a);
        }
    }
    fn set(pix: &mut Pixel_set, x: u16, y: u16, a: Pixel){
        let mut xl = x;
        let mut yl = y;
        if(xl >= pix.w){ xl = pix.w -1;}
        else if(xl < 0) {xl = 0;}
        if(yl >= pix.h) {yl = pix.h - 1;}
        else if(yl < 0) {yl = 0;}
        pix.data[((yl as u32 * pix.w as u32) + xl as u32) as usize] = a;
    }
    fn draw_line(pix: &mut Pixel_set, x: f32, y: f32, x1:f32, y1: f32, a: Pixel){
        let alphax: f32 = (x1 - x) / (y1 - y);
        let alphay: f32 = (y1 - y) / (x1 - x);
        let mut xl:f32 = x;
        let mut yl:f32 = y;
        loop{
            Pixel_set::set(pix, xl as u16, yl as u16, a);
            xl += alphax;
            yl += alphay;
            if(xl > x1 || yl > y1){
                break;
            }
        }
    }
    fn draw_circle(pix: &mut Pixel_set, x: f32, y: f32, radius: f32, px: Pixel) -> (){
        let mut acx: f32 = x - radius as f32;
        let mut acy: f32 = 0.;
        let dx: f32 = (radius * 2.) / 100.;
        loop{
            unsafe {
                acy = (radius * radius - x * x).sqrt();
            }
            acx = acx + dx;
            Pixel_set::set(pix, acy as u16, acx as u16, px);
            if(acx > x + radius){
                break;
            }
        }
    }
    fn draw_rect(pix: &mut Pixel_set, x1: u16, y1: u16, x2: u16, y2: u16, pixl: Pixel) -> (){
        for i in x1..x2{
            for j in y1..y2{
                Pixel_set::set(pix, i, j, pixl);
            }
        }
    }
    fn draw_gradient(pix: &mut Pixel_set, px: Pixel, px1: Pixel, dr: dir){
        for i in 0..pix.w-1{
            Pixel_set::draw_line(pix, i as f32, 0., (i + 1) as f32, pix.h as f32, Pixel::merge(px, px1, i as f32 / pix.w as f32) );
        }
    }
}

fn create_empty_tga_file(name: &str, pix: Pixel_set){
    let w = pix.w;
    let h = pix.h;
    let mut file = match std::fs::File::create(format!("{}.tga", name)){
        Ok(file) => file,
        Err(e) => {println!("error"); return;}
    };
    let header:[u8; 18] =
        [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, (w & 0x00FF) as u8, ((h & 0xFF00) >> 8) as u8, (h & 0x00FF) as u8, ((h & 0xFF00) >> 8) as u8, 32, 0];
    let mut img = vec![0u8; w as usize * h as usize * 4 as usize];
    let mut j = 0;
    for i in 0u64..(w as u64 *h as u64){
        img[j] = pix.data[i as usize].b;
        img[j + 1] = pix.data[i as usize].g;
        img[j + 2] = pix.data[i as usize].r;
        img[j + 3] = pix.data[i as usize].a;
        j = j + 4;
    }
    file.write(&header);
    file.write(&img);
}

const red: Pixel = Pixel {r: 255, g: 0, b: 0, a: 0};
const green: Pixel = Pixel {r: 0, g: 255, b: 0, a: 0};
const blue: Pixel = Pixel {r: 0, g:0, b: 255, a: 0};
fn main(){
    let mut set: Pixel_set = Pixel_set::new(312, 312);
    Pixel_set::fill(&mut set, Pixel {r:255, g:255, b:255, a:255});
    Pixel_set::draw_gradient(&mut set, red, green, dir::Ox);
    create_empty_tga_file("filed", set);
}