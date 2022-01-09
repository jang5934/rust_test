use std::io;

fn main() {
    // 아래와 같이 mut 키워드를 추가하지 않으면 해당 변수는 변경될 수 없다.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 상수는 변수와 다르게 힙 영역에 할당되지 않는다.
    // 언더바는 자릿수를 표시한다.
    const MAX_POINTS: u32 = 100_000;

    // 쉐도잉
    // 메모리를 재사용하는 것이 아니라, 동일한 변수명을 재사용하는 것이라고 한다.
    // 아래와 같이 동일한 변수명을 재사용하면서 값을 재할당 하는 경우에는 mut이 없어도 된다.
    // 코드 블럭에도 영향을 받는다고 한다.
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of x is: {}", y);

    // 아래와 같이 데이터형이 달라져도 쉐도잉이 가능하다 단,
    let spaces_1 = "    ";
    let spaces_1 = spaces_1.len();

    // 아래와 같이 let이 빠지면 불변 변수를 수정한 것이기 때문에 컴파일러에서 오류가 난다.
    let spaces_2 = "      ";
    /*
    spaces_2 = spaces_2.len();
    */

    // 데이터 유형
    // u32 와 같은 키워드를 변수 우측에 붙여주는데, 이것이 데이터 타입.
    // inferrable인 경우에는 굳이 달아주지 않아도 되지만, 일단 달아주는 것이 좋은 것 같다.
    let guess: u32 = "42".parse().expect("Not a number!");

    // 크게 스칼라, 복합(컴파운드) 유형으로 나눌 수 있다.

    // 스칼라 타입으로는
    // 부호 있는 정수 i8, i16, ..., i128 그리고 isize(포인터 크기)
    // 부호 없는 정수 u8, u16, ..., u128 그리고 usize(포인터 크기)
    // 부동 소수점 f32, f64
    // 'a', 'α' 그리고 '∞' 와 같은 문자형 char (각 4바이트)
    // true, false 를 의미하는 bool
    // 그리고 오직 빈 튜플 타입만 값으로 가질 수 있는 타입 ()

    // 복합 타입으로는
    // 배열 [1, 2, 3]
    // 튜플 (1, true)

    // 그리고 2, 8, 10 그리고 16진수 표현 포멧은 아래와 같다.
    // 2진수 : 0b1_0000 <= 1 0000_2 <= 10000_2 <= 16_10
    // 8진수 : 0o11 <= 11_8 <= 9_10
    // 10진수 : 123_456 <= 123456_10
    // 16진수 : 0xff <= ff_16 <= 255_10

    // 이처럼 '_' 언더바를 활용해 4자리씩 혹은 3자리씩 끊어 읽기 편하게 돕는다.

    // 마지막으로 바이트는 아래와 같이 표현하면 된다.
    // b'A'

    // 복합 타입을 다루는 방법들에 대해 간단히 알아보자.
    // 일단 기본적으로 튜플로 값을 묶게 되면 해당 값을 변경할 수 없고, 튜플의 크기 또한 변경이 불가능하다.

    // 아래와 같이 데이터 타입을 정해서 선언하는 방법이 있고,
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 아니면 아래처럼 안 정하고 선언하기도 한다.
    // 아래와 같이 튜플의 값을 개별 대입하기도 한다.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    // 그리고 아래와 같이 각 튜플 요소에 접근할 수도 있다.
    println!("The value of y equals to: {}", tup.1);

    // 배열을 다루는 방법에 대해 간단히 알아보자.
    // 아래와 같이 선언이 가능하다. 단, 위의 튜플과 마찬가지로 크기라 고정된다.
    // 사실 이렇게 고정되면 스택에 배치됨으로써 성능상의 이점을 가질 수 있게 된다.
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 단, 이러한 배열의 기능이 필요하면서 길이가 변할 수 있어야 한다면 벡터를 써야 한다.

    // 배열 선언 방법은 위 말고도 다양하게 있다.
    // i32짜리 6칸
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // 3으로 4칸 = [3, 3, 3, 3]
    let arr = [3; 4];

    // random access 방법은 일반적인 방법과 같다.
    arr[1]; // = 3
}

// 아래의 테스트 함수는 배열의 인덱스를 벗어나는 경우를 테스트하기 위해 작성된 것이다.
// outOfIdx 오류를 낸다.
fn randomAccessTest() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

// 그리고 함수의 인자에도 데이터 형을 선언하여 보다 안정성을 높일 수 있다
fn testFunction(a: u32, b: i32) {
    println!("a : {}", a);
    println!("b : {}", b);
}

// 명령문에 대해 알아보자.
fn testCommand() {
    // 아래와 같은 코드는 기본적으로 반환값이 없으므로,
    let a = 16;
    // 아래와 같이 사용이 불가하다.
    // let b = (let a = 16);

    // 그러나 아래는 가능하다.
    let b = {
        let a = 20;
        a + 3;
    };
}

// 다르게 응용해보면 아래와 같이 응용할 수 있을 것이다.
// 아래의 six() 함수는 아무런 리턴이 선언되지 않았지만 rust의 특성상 맨 마지막 값을 반환하게 된다.
// 그리고 i32 는 리턴 타입을 의미한다.
fn six() -> i32 {
    6
}
fn testCommand_2() {
    let x = six();

    println!("six is {}", x);
}

// 주석(코멘트)
// 지금도 잘 쓰고 있지만 // 이거나 또는 /* */ 를 쓴다.

// control command, 제어 방식에 대해 알아본다.
// 기본적으로 아래와 같으며, 일반적인 언어들의 문법과 비슷하지만 조건문 쪽을 괄호로 감싸지 않는다.
fn controlCommand_1() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number != 0 {
        println!("number is not zero");
    }

    // 아래와 같은 조건문은 오류가 난다.
    // 조건문의 반환값은 오로지 bool 값만 받는다.
    /*
    if number {
        println!("number is three");
    }
    */
    // 아래와 같이 쓰기도 한다.
    let condition = true;
    let newNumber = if condition { 1 } else { 2 };
    println!("The value of number is: {}", newNumber);
    // 다만 위의 변수에 들어가는 값이 분기마다 다른 데이터 타입이라면 오류가 발생한다.
    /*
    let newNumber = if condition { 1 } else { "two" };
    */
    println!("The value of number is: {}", newNumber);

    // 그 외에도 match를 활용한 분기 처리 커맨드를 제공해주는 것으로 알고 있는데 이건 나중에 공부해보자.
    // 지금 공부중인 페이지에 없기 때문이다.
}

// 반복문
fn loopTest() {
    // rust에는 loop, for, while 등의 반복문이 있다.

    // 아래는 무한 반복문이다.
    // 아래의 무한 반복문 때문에 아래의 코드들이 unreachable 하기 때문에
    // 이건 잠깐 주석처리 하겠다.
    /*
    loop {
        println!("inf loop");
    }
    */

    // 그러나 아래와 같이 활용하기도 한다.
    // 맨 마지막 값을 반환한다고 위에서 배웠듯, result에는 20이 들어간다.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // 20
    println!("The result is {}", result);

    // 이번에는 while에 대해 알아본다.
    let mut whileNum = 3;
    while whileNum != 0 {
        println!("{}!", whileNum);
        whileNum -= 1;
    }

    // 그리고 아래와 같이 while문으로 배열을 루프 돌 수 있기도 하지만 모양새가 별로라고 한다.
    let whileArr = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("The vlaue is: {}", whileArr[idx]);
        idx += 1;
    }

    // 그래서 아래와 같이 for 문으로 루프를 돈다고 한다.
    for element in whileArr.iter() {
        println!("the value is: {}", element);
    }

    // 아니면 아래와 같이 사용할 수도 있다고 한다.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
