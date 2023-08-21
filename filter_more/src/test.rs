use crate::bmp::{RGBTriple};

fn pixel(r: u8, g: u8, b: u8) -> RGBTriple {
    RGBTriple::new(r, g, b)
}

fn get_img1() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(0xff, 0, 0); 3],
        vec![pixel(0, 0xff, 0); 3],
        vec![pixel(0, 0, 0xff); 3],
    ]
}

fn get_img2() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(10, 20, 30), pixel(40, 50, 60), pixel(70, 80, 90)],
        vec![pixel(110, 130, 140), pixel(120, 140, 150), pixel(130, 150, 160)],
        vec![pixel(200, 210, 220), pixel(220, 230, 240), pixel(240, 250, 255)],
    ]
}

fn get_img3() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(10, 20, 30), pixel(40, 50, 60), pixel(70, 80, 90), pixel(100, 110, 120)],
        vec![pixel(110, 130, 140), pixel(120, 140, 150), pixel(130, 150, 160), pixel(140, 160, 170)],
        vec![pixel(195, 204, 213), pixel(205, 214, 223), pixel(225, 234, 243), pixel(245, 254, 253)],
        vec![pixel(50, 28, 90), pixel(0, 0, 0), pixel(255, 255, 255), pixel(85, 85, 85)]
    ]
}

fn get_img4() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(0, 10, 25), pixel(0, 10, 30), pixel(40, 60, 80)],
        vec![pixel(20, 30, 90), pixel(30, 40, 100), pixel(80, 70, 90)],
        vec![pixel(20, 20, 40), pixel(30, 10, 30), pixel(50, 40, 10)]
    ]
}

fn get_img5() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(0, 10, 25), pixel(0, 10, 30), pixel(40, 60, 80), pixel(50, 60, 80)],
        vec![pixel(20, 30, 90), pixel(30, 40, 100), pixel(80, 70, 90), pixel(80, 80, 90)],
        vec![pixel(20, 20, 40), pixel(30, 10, 30), pixel(50, 40, 10), pixel(50, 40, 100)],
        vec![pixel(50, 20, 40), pixel(50, 20, 40), pixel(50, 40, 80), pixel(50, 40, 80)]
    ]
}
fn get_row2() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(255, 0, 0), pixel(0, 0, 255)]
    ]
}

fn get_row3() -> Vec<Vec<RGBTriple>> {
    vec![
        vec![pixel(255, 0, 0), pixel(0, 255, 0), pixel(0, 0, 255)]
    ]
}

fn assert_images_equal(a: Vec<Vec<RGBTriple>>, b: Vec<Vec<RGBTriple>>) {
    assert_eq!(a.len(), b.len());
    for (row_a, row_b) in a.iter().zip(b.iter()) {
        assert_eq!(row_a, row_b);
    }
}

mod grey_scale {
    use crate::filter::grey_scale;
    use crate::test::{pixel, assert_images_equal, get_img1, get_img2, get_img3};

    #[test]
    fn greyscale_single_pixel() {
        let mut img = vec![
            vec![pixel(20, 40, 90)],
        ];
        
        grey_scale(&mut img);
        
        let expected = vec![
            vec![pixel(50, 50, 50)], // This is a mock value; replace with the correct grayscale value.
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn greyscale_rounding() {
        let mut img = vec![
            vec![pixel(27, 28, 28)],
        ];
        
        grey_scale(&mut img);
        
        let expected = vec![
            vec![pixel(28, 28, 28)], // Mock value.
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn greyscale_grey() {
        let mut img = vec![
            vec![pixel(50, 50, 50)],
        ];

        grey_scale(&mut img);

        let expected = vec![
            vec![pixel(50, 50, 50)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn greyscale_multi() {
        let mut img = get_img1();

        grey_scale(&mut img);

        let expected = vec![
            vec![pixel(85, 85, 85), pixel(85, 85, 85), pixel(85, 85, 85)],
            vec![pixel(85, 85, 85), pixel(85, 85, 85), pixel(85, 85, 85)],
            vec![pixel(85, 85, 85), pixel(85, 85, 85), pixel(85, 85, 85)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn greyscale3x3() {
        let mut img = get_img2();

        grey_scale(&mut img);

        let expected = vec![
            vec![pixel(20, 20, 20), pixel(50, 50, 50), pixel(80, 80, 80)],
            vec![pixel(127, 127, 127), pixel(137, 137, 137), pixel(147, 147, 147)],
            vec![pixel(210, 210, 210), pixel(230, 230, 230), pixel(248, 248, 248)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn greyscale4x4() {
        let mut img = get_img3();

        grey_scale(&mut img);
        
        let expected = vec![
            vec![pixel(20, 20, 20), pixel(50, 50, 50), pixel(80, 80, 80), pixel(110, 110, 110)],
            vec![pixel(127, 127, 127), pixel(137, 137, 137), pixel(147, 147, 147), pixel(157, 157, 157)],
            vec![pixel(204, 204, 204), pixel(214, 214, 214), pixel(234, 234, 234), pixel(251, 251, 251)],
            vec![pixel(56, 56, 56), pixel(0, 0, 0), pixel(255, 255, 255), pixel(85, 85, 85)],
        ];
            
        assert_images_equal(img, expected);
    }
}

mod reflect {
    use crate::filter::reflect;
    use crate::test::{pixel, assert_images_equal, get_row2, get_row3, get_img1, get_img2, get_img3};

    #[test]
    fn reflect_row_2() {
        let mut img = get_row2();
        reflect(&mut img);
        
        let expected = vec![
            vec![pixel(0, 0, 255), pixel(255, 0, 0)], // This is a mock value; replace with the correct grayscale value.
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn reflect_row_3() {
        let mut img = get_row3();

        reflect(&mut img);
        
        let expected = vec![
            vec![pixel(0, 0, 255), pixel(0, 255, 0), pixel(255, 0, 0)], // This is a mock value; replace with the correct grayscale value.
        ];
        
        assert_images_equal(img, expected);
    }


    #[test]
    fn reflect_simple() {
        let mut img = get_img1();

        reflect(&mut img);

        let expected = vec![
            vec![pixel(255, 0, 0), pixel(255, 0, 0), pixel(255, 0, 0)],
            vec![pixel(0, 255, 0), pixel(0, 255, 0), pixel(0, 255, 0)],
            vec![pixel(0, 0, 255), pixel(0, 0, 255), pixel(0, 0, 255)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn reflect_3x3() {
        let mut img = get_img2();

        reflect(&mut img);

        let expected = vec![
            vec![pixel(70, 80, 90), pixel(40, 50, 60), pixel(10, 20, 30)],
            vec![pixel(130, 150, 160), pixel(120, 140, 150), pixel(110, 130, 140)],
            vec![pixel(240, 250, 255), pixel(220, 230, 240), pixel(200, 210, 220)]
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn reflect_4x4() {
        let mut img = get_img3();

        reflect(&mut img);

        let expected = vec![
            vec![pixel(100, 110, 120), pixel(70, 80, 90), pixel(40, 50, 60), pixel(10, 20, 30)],
            vec![pixel(140, 160, 170), pixel(130, 150, 160), pixel(120, 140, 150), pixel(110, 130, 140)],
            vec![pixel(245, 254, 253), pixel(225, 234, 243), pixel(205, 214, 223), pixel(195, 204, 213)],
            vec![pixel(85, 85, 85), pixel(255, 255, 255), pixel(0, 0, 0), pixel(50, 28, 90)]
        ];
        
        
        assert_images_equal(img, expected);
    }
}
