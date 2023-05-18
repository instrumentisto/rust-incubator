//! Extension trait for an [`Iterator`].
//!
//! Stolen from [`itertools` crate][0].
//!
//! [0]: https://docs.rs/itertools/latest/src/itertools/lib.rs.html#2078-2136

use std::fmt;

use self::format::{Format, FormatWith};

/// Extension trait for an [`Iterator`].
pub trait MyIteratorExt: Iterator {
    /// Format all iterator elements, separated by `sep`.
    ///
    /// All elements are formatted (any formatting trait)
    /// with `sep` inserted between each element.
    ///
    /// **Panics** if the formatter helper is formatted more than once.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
    ///
    /// let data = [1.1, 2.71828, -3.];
    /// assert_eq!(
    ///     format!("{:.2}", data.iter().format(", ")),
    ///            "1.10, 2.72, -3.00");
    /// ```
    fn format(self, sep: &str) -> Format<Self>
    where
        Self: Sized,
    {
        format::new_format_default(self, sep)
    }

    /// Format all iterator elements, separated by `sep`.
    ///
    /// This is a customizable version of [`.format()`](MyIteratorExt::format).
    ///
    /// The supplied closure `format` is called once per iterator element,
    /// with two arguments: the element and a callback that takes a
    /// `&Display` value, i.e. any reference to type that implements `Display`.
    ///
    /// Using `&format_args!(...)` is the most versatile way to apply custom
    /// element formatting. The callback can be called multiple times if needed.
    ///
    /// **Panics** if the formatter helper is formatted more than once.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
    ///
    /// let data = [1.1, 2.71828, -3.];
    /// let data_formatter = data.iter().format_with(", ", |elt, f| f(&format_args!("{:.2}", elt)));
    /// assert_eq!(format!("{}", data_formatter),
    ///            "1.10, 2.72, -3.00");
    ///
    /// // .format_with() is recursively composable
    /// let matrix = [[1., 2., 3.],
    ///               [4., 5., 6.]];
    /// let matrix_formatter = matrix.iter().format_with("\n", |row, f| {
    ///     f(&row.iter().format_with(", ", |elt, g| g(&elt)))
    /// });
    /// assert_eq!(matrix_formatter.to_string(), "1, 2, 3\n4, 5, 6");
    /// ```
    fn format_with<F>(self, sep: &str, format: F) -> FormatWith<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item, &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result) -> fmt::Result,
    {
        format::new_format(self, sep, format)
    }
}

impl<T> MyIteratorExt for T where T: Iterator {}

mod format {
    use std::{cell::RefCell, fmt};

    /// Format all iterator elements lazily, separated by `sep`.
    ///
    /// The format value can only be formatted once, after that the iterator is
    /// exhausted.
    ///
    /// See [`.format_with()`](crate::MyIteratorExt::format_with) for more information.
    #[derive(Clone)]
    pub struct FormatWith<'a, I, F> {
        sep: &'a str,
        /// FormatWith uses interior mutability because Display::fmt takes &self.
        inner: RefCell<Option<(I, F)>>,
    }

    /// Format all iterator elements lazily, separated by `sep`.
    ///
    /// The format value can only be formatted once, after that the iterator is
    /// exhausted.
    ///
    /// See [`.format()`](crate::MyIteratorExt::format)
    /// for more information.
    #[derive(Clone)]
    pub struct Format<'a, I> {
        sep: &'a str,
        /// Format uses interior mutability because Display::fmt takes &self.
        inner: RefCell<Option<I>>,
    }

    pub fn new_format<I, F>(iter: I, separator: &str, f: F) -> FormatWith<'_, I, F>
    where
        I: Iterator,
        F: FnMut(I::Item, &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result) -> fmt::Result,
    {
        FormatWith {
            sep: separator,
            inner: RefCell::new(Some((iter, f))),
        }
    }

    pub fn new_format_default<I>(iter: I, separator: &str) -> Format<'_, I>
    where
        I: Iterator,
    {
        Format {
            sep: separator,
            inner: RefCell::new(Some(iter)),
        }
    }

    impl<'a, I, F> fmt::Display for FormatWith<'a, I, F>
    where
        I: Iterator,
        F: FnMut(I::Item, &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let (mut iter, mut format) = match self.inner.borrow_mut().take() {
                Some(t) => t,
                None => panic!("FormatWith: was already formatted once"),
            };

            if let Some(fst) = iter.next() {
                format(fst, &mut |disp: &dyn fmt::Display| disp.fmt(f))?;
                iter.try_for_each(|elt| {
                    if !self.sep.is_empty() {
                        f.write_str(self.sep)?;
                    }
                    format(elt, &mut |disp: &dyn fmt::Display| disp.fmt(f))
                })?;
            }
            Ok(())
        }
    }

    impl<'a, I> Format<'a, I>
    where
        I: Iterator,
    {
        fn format<F>(&self, f: &mut fmt::Formatter, mut cb: F) -> fmt::Result
        where
            F: FnMut(&I::Item, &mut fmt::Formatter) -> fmt::Result,
        {
            let mut iter = match self.inner.borrow_mut().take() {
                Some(t) => t,
                None => panic!("Format: was already formatted once"),
            };

            if let Some(fst) = iter.next() {
                cb(&fst, f)?;
                iter.try_for_each(|elt| {
                    if !self.sep.is_empty() {
                        f.write_str(self.sep)?;
                    }
                    cb(&elt, f)
                })?;
            }
            Ok(())
        }
    }

    macro_rules! impl_format {
        ($($fmt_trait:ident)*) => {
            $(
                impl<'a, I> fmt::$fmt_trait for Format<'a, I>
                where
                    I: Iterator,
                    I::Item: fmt::$fmt_trait,
                {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        self.format(f, fmt::$fmt_trait::fmt)
                    }
                }
            )*
        }
    }

    impl_format! {
        Display Debug UpperExp LowerExp UpperHex LowerHex Octal Binary Pointer
    }
}
