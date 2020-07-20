# OS Tutorial Summer of Code

## 7.4

#### 复习 Rust 编程之道

* 复习 Rust 编程之道： 之前有粗略地度过Rust编程之道，现在可以再复习一下。
* 开始 Rustling 练习： 目前完成到 `move_semantics2`

```rust
// 使用enum的匿名结构体进行匹配
match message {
	Message::Move{x, y} => self.move_position(Point{x, y}),
	....
}

// 使用 std::cmp::max 进行比较
pub fn bigger(a: i32, b: i32) -> i32 {
    cmp::max(a,b)
}
```

## 7.5

* Rustling练习全部完成

```rust
// 错误处理：直接使用 Box<dyn error::Error> 处理
fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;
		......
    Ok(answer)
}


// 迭代器：向特定目标 collect
iterator.collect::<String>()

// 迭代器：可以用来代替循环
pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}

// FromStr 和 .parse<T>()
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        ....
    }
}
"something".parse<Person>()

// should_panic 测试
#[should_panic]
```

## 7.6 

* 忙于我们学校的 OS 实验，与 Rust 相关：[rVisor - 使用Rust编写内核中的安全沙箱](https://github.com/OSH-2020/x-chital)

## 7.7

* 做了两道算法题。
* 同样忙于实验。

## 7.8

* 忙于实验。

## 7.9 

* 受 [MashPlant](https://github.com/MashPlant/lalr1) 大神影响，对编译比较感兴趣，尝试使用 Rust 编写一个编译式的正则表达式引擎，如果时间充足，可以尝试编写 lalr1 生成器。
* 学校的 OS 实验大体完成。

## 7.10

* 之前的正则表达式引擎，已经完成了 DFA 的生成（下图为 `([A-Z]*|A[a-z]*)H` 的 Dfa）。[项目地址](https://github.com/DnailZ/simple-regex)

![image-20200720235129904](/Users/dnailz/Course/os-tutorial-summer-of-code/README.assets/image-20200720235129904.png)

* 继续进行 Rust 的刷题。感觉想要用 Rust 优雅地刷题还是比较困难的，无法避免地需要使用大量的下标运算，Rust 会检查其是否越界，可能会带来性能损失，而且下标一旦用错了就容易panic，最好想一些办法避免。



