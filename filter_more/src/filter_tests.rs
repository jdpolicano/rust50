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

fn assert_pixels_equal(a: RGBTriple, b: RGBTriple) {
    assert_eq!(a.rgb_blue, b.rgb_blue);
    assert_eq!(a.rgb_green, b.rgb_green);
    assert_eq!(a.rgb_red, b.rgb_red);
}

mod grey_scale {
    use crate::filter::grey_scale;
    use crate::filter_tests::{pixel, assert_images_equal, get_img1, get_img2, get_img3};

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
    use crate::filter_tests::{pixel, assert_images_equal, get_row2, get_row3, get_img1, get_img2, get_img3};

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

mod blur {
    use crate::filter::blur;
    use crate::filter_tests::{pixel, assert_images_equal, assert_pixels_equal, get_img2, get_img3};

    #[test]
    fn blur_middle() {
        let mut img = get_img2();

        blur(&mut img);

        let expected = pixel(127, 140, 149);
    
        assert_pixels_equal(img[1][1].clone(), expected);
    }

    #[test]
    fn blur_edge() {
        let mut img = get_img2();

        blur(&mut img);

        let expected = pixel(80, 95, 105);
        
        assert_pixels_equal(img[0][1].clone(), expected);
    }

    #[test]
    fn blur_corner() {
        let mut img = get_img2();

        blur(&mut img);

        let expected = pixel(70, 85, 95);
        
        assert_pixels_equal(img[0][0].clone(), expected);
    }

    #[test]
    fn blur_3x3() {
        let mut img = get_img2();

        blur(&mut img);
        
        let expected = vec![
            vec![pixel(70, 85, 95), pixel(80, 95, 105), pixel(90, 105, 115)],
            vec![pixel(117, 130, 140), pixel(127, 140, 149), pixel(137, 150, 159)],
            vec![pixel(163, 178, 188), pixel(170, 185, 194), pixel(178, 193, 201)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn blur_4x4() {
        let mut img = get_img3();

        blur(&mut img);

        let expected = vec![
            vec![pixel(70, 85, 95), pixel(80, 95, 105), pixel(100, 115, 125), pixel(110, 125, 135)],
            vec![pixel(113, 126, 136), pixel(123, 136, 145), pixel(142, 155, 163), pixel(152, 165, 173)],
            vec![pixel(113, 119, 136), pixel(143, 151, 164), pixel(156, 166, 171), pixel(180, 190, 194)],
            vec![pixel(113, 112, 132), pixel(155, 156, 171), pixel(169, 174, 177), pixel(203, 207, 209)],
        ];
        
        assert_images_equal(img, expected);
    }
}

mod edges {
    use crate::filter::edges;
    use crate::filter_tests::{pixel, assert_images_equal, assert_pixels_equal, get_img4, get_img5};

    #[test]
    fn edges_middle() {
        let mut img = get_img4();

        edges(&mut img);

        let expected = pixel(210, 150, 60);
    
        assert_pixels_equal(img[1][1].clone(), expected);
    }

    #[test]
    fn edges_edge() {
        let mut img = get_img4();

        edges(&mut img);

        let expected = pixel(213, 228, 255);
        
        assert_pixels_equal(img[0][1].clone(), expected);
    }

    #[test]
    fn edges_corner() {
        let mut img = get_img4();

        edges(&mut img);

        let expected = pixel(76, 117, 255);
        
        assert_pixels_equal(img[0][0].clone(), expected);
    }

    #[test]
    fn edges_3x3() {
        let mut img = get_img4();

        edges(&mut img);
        
        let expected = vec![
            vec![pixel(76, 117, 255), pixel(213, 228, 255), pixel(192, 190, 255)],
            vec![pixel(114, 102, 255), pixel(210, 150, 60), pixel(103, 108, 255)],
            vec![pixel(114, 117, 255), pixel(200, 197, 255), pixel(210, 190, 255)],
        ];
        
        assert_images_equal(img, expected);
    }

    #[test]
    fn edges_4x4() {
        let mut img = get_img5();

        edges(&mut img);

        let expected = vec![
            vec![pixel(76, 117, 255), pixel(213, 228, 255), pixel(255, 255, 255), pixel(255, 255, 255)],
            vec![pixel(114, 102, 255), pixel(210, 150, 60), pixel(177, 171, 156), pixel(250, 247, 255)],
            vec![pixel(161, 89, 255), pixel(126, 128, 181), pixel(114, 170, 192), pixel(247, 220, 192)],
            vec![pixel(148, 71, 156), pixel(133, 100, 121), pixel(181, 148, 212), pixel(212, 170, 255)],
        ];
        
        assert_images_equal(img, expected);
    }

}
