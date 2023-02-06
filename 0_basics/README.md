Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?

[x] 
    Раст поддерживает многопоточность из коробки, по умолчанию синхронен, с возможностью асинхронного программирования с помощью крейтов в виде tokio, async-std и других.
    
- What runtime [Rust] has? Does it use a GC (garbage collector)?

[x] 
    Имеет нулевой рантайм, не имеет сборщика мусора, вместо этого использует концепцию "владения и заимствования"
    
- What statically typing means? What is a benefit of using it?

[x] 
    Статическая типизация дает самый простой машинный код, = она удобна для языков, дающих исполняемые файлы операционных систем, или JIT-комплируемые промежуточные коды. Многие ошибки исключаются на стадии компиляции(в отличии от динамически типизированных языков, где многие ошибки отлавливаются во время исполнения) 
    
- What are generics and parametric polymorphism? Which problems do they solve?

[x] 
    Дженерики - используются для того, что бы функция работала с несколькими типами аргументов, исключают необходимость создавать копии "идентичных" функций для обработки каждого конкретного типа данных.
    
    fn some_here<T>(x: T) {}
    
    
    Дженерики испозьзуют static dispath
    
    Dynamic dispath - Сокращают размер двоичного файла, но приводят к снижению производительности. Компилятор требует использовать ключевое слово "dyn"

- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?

- What are static and dynamic dispatches? Which should I use, and when?

- What is a crate and what is a module in Rust? How do they differ? How are the used?

[x]
    Крейт - некая библиотека, которую мы можем вызывать из вне (extern crate)
    Модуль - обозначается ключевым словом "mod", чаще всего использется для разделения программы на множество модулей (модульное программирование)
    
- What are move semantics? What are borrowing rules? What is the benefit of using them?

[x] 
    Семантика перемещения - move semantic 
        В раст все типы являются перемещаемями, и все операции перемещения представляют собой битовую копию исходных данных в новое место.
        Перемещение - операция по уболчанию, мы должны явно указывать иное (передача ссылки, &mut ссылки)
    Польза - помогает писать полее безопасный по памяти код.
    
- What is immutability? What is the benefit of using it?

[x]
    Неизмениемость - помогает лучше контролировать поведение, позволяет писать более читабельный код, вместо mut переменных часто можно использовать "замещение"
    
- What is cloning? What is copying? How do they compare?
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
- What is an iterator? What is a collection? How do they differ? How are they used?

[x] 
    iter - это некий процесс, для перебора значений. В rust итераторы помогают создать процесс зацикливания, мы можем перебирать массив значений.
    collection - это структуры данных, предоставляемые стандартной библиотекой Rust. Хранят информацию в виде последовательностей или групп. Могут содержать несколько значений одновременно. (Vec, HashMap) 
    Обычно Vec - выбор по умолчанию для совместного хранения элементов.
    
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?

[x]
    Макросы - обозначаются "!"в конце, как print!, println!. Помогают отхватывать множество шаблонов повторного использования кода. Помогают расширить синтакис языка. Rust включает в себя 2 вида макросов - процедурные и декларативные. Процедурные - легко включить в код. Декларативные - лучше подойдут для генерации кода, который не впишется в окружающий код. В остальном их работа схожа. Создать макрос можно через macro_rules!. 
    
    macro_rules! hello_world {
        () => {
            println!("Hello, World!");
        };
    }
    
- How code is tested in [Rust]? Where should you put tests and why?

[x]
    Тесты в раст - cargo test
    обьявляем модуль с помощью #[cfg(test)]
    пример:
    
    #[cfg(test)]
    mod tests {
        #[test]
        fn test1() {
            let resutl = 2+2;
            assert_eq!(result, 4);
        }
    }

- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?

[x] 
    String - относится к предварительно выделенному тексту, хранящемуся вместе с машинным кодом программы.
    &str - ссылка на набор текста, принадлежащего кому-то другому.
    
- What are lifetimes? Which problems do they solve? Which benefits do they give?

[x] 
    Это время жизни переменной, решают проблемы утечки памяти, как пример: доступ по указателю к переменной, которая была освобождена.
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?

After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
