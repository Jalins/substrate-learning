fn main() {
    let t = Triangle{
        bottom: 32.0,
        height: 34.0,
    };
    cal_area(t);

    let c = Circle{
        diameter: 12.0,
    };
    cal_area(c);

    let s = Square{
        long: 64.0,
        width: 80.0,
    };
    cal_area(s)
}

// 定义一个trait，包含一个未实现的方法
trait   Area {
   fn calculate(&self) -> f64;
}

// ================================================ 三角形实现 ======================================================
// 定义三角形结构体
struct Triangle {
    bottom: f64,
    height:f64,
}

// 实现Area
impl Area for Triangle {
    fn calculate(&self) -> f64 {
        let res =  self.bottom * self.height / 2.0;
        println!("三角形的面积为：{}", res);
        return  res;

    }
}

// ================================================ 圆形实现 ======================================================
// 定义圆形结构体
struct Circle {
    diameter: f64,
}

// 实现Area
impl Area for Circle {
    fn calculate(&self) -> f64 {
        let pi = std::f64::consts::PI;
        let res = pi * self.diameter.powf(2.0);
        println!("圆形的面积为：{}", res);
        return  res;

    }
}

// ================================================ 正方形实现 ======================================================

struct Square {
    width: f64,
    long : f64,
}

// 实现Area
impl Area for Square {
    fn calculate(&self) -> f64 {
        let res = self.long * self.width;
        println!("正方形的面积为：{}", res);
        return  res;

    }
}

fn cal_area<T:Area>(area: T){
    area.calculate();
}
