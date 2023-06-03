//! # 排序算法
//!
//! 该项目为切片类型实现了下列排序算法：
//!
//! * 冒泡排序
//! * 选择排序
//! * 插入排序
//! * 快速排序
//!

/// # 冒泡排序
///
pub trait BubbleSort {
    /// 对当前对象进行冒泡排序
    fn bubble_sort(&mut self);
}

// 对于任意元素类型实现了 Ord 接口的切片类型
// 实现冒泡排序
impl<T: Ord> BubbleSort for [T] {
    fn bubble_sort(&mut self) {
        let mut swapped = true;
        // 需要迭代的次数等于切片长度
        for i in 0..self.len() {
            // [ unsorted | sorted ]
            if !swapped {
                // 如果上一轮没有交换过，表示排序完成
                // 可以提前结束
                break;
            }
            swapped = false;
            for j in 0..self.len() - i - 1 {
                // 如果当前元素大于后面的元素，交换
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                    swapped = true;
                }
            }
        }
    }
}

/// # 选择排序
///
pub trait SelectionSort {
    /// 对当前对象进行选择排序
    fn selection_sort(&mut self);
}

impl<T: Ord> SelectionSort for [T] {
    fn selection_sort(&mut self) {
        // [ sorted | unsorted ]
        for i in 0..self.len() {
            // 找到最小的元素的索引
            let Some((index, _)) = self[i..].iter()
                .enumerate()
                .min_by_key(|(_, value)| *value) else {
                break;
            };

            // [ sorted | unsorted .. (index, value) .. unsorted ]
            // 将 [ unsorted .. (index, value) ] 这部分向右 rotate 即可
            let to_rotate = &mut self[i..=i + index];
            if !to_rotate.is_empty() {
                to_rotate.rotate_right(1);
            }
        }
    }
}

/// # 插入排序
pub trait InsertionSort {
    /// 对当前对象进行插入排序
    fn insertion_sort(&mut self);
}

impl<T: Ord> InsertionSort for [T] {
    fn insertion_sort(&mut self) {
        for i in 0..self.len() {
            // [ sorted | unsorted ]
            // 依次将 unsorted 的部分，插入到 sorted 中合适的位置

            // 0. 使用二分查找，找到插入位置
            let (Ok(index) | Err(index)) = self[0..i].binary_search(&self[i]);
            // [ sorted .. index .. sorted | i | unsorted ]
            // 将 [ index .. sorted | i ] 这部分，向右 rotate 即可
            let to_rotate = &mut self[index..=i];
            if !to_rotate.is_empty() {
                to_rotate.rotate_right(1);
            }
        }
    }
}

/// # 快速排序
pub trait QuickSort {
    /// 对当前对象进行快速排序
    fn quick_sort(&mut self);
}

impl<T: Ord> QuickSort for [T] {
    fn quick_sort(&mut self) {
        match self {
            [] | [_] => {}
            [first, second] => {
                if first > second {
                    std::mem::swap(first, second);
                }
            }
            [pivot, rest @ ..] => {
                let mut left = 0;
                // 按照上面的模式匹配，rest 的长度必然大于 0
                let mut right = rest.len() - 1;
                while left <= right {
                    if &rest[left] <= pivot {
                        left += 1;
                    } else if &rest[right] > pivot {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    } else {
                        rest.swap(left, right);
                        left += 1;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                }

                let stop = left + 1;
                self.swap(0, stop - 1);
                self[..stop - 1].quick_sort();
                self[stop..].quick_sort();
            }
        }
    }
}

/// # 测试工具
#[cfg(test)]
pub(crate) mod tests {
    use crate::*;
    use rand::distributions::{Distribution, Standard};
    use std::fmt::Debug;

    /// # 测试排序函数是否能够正确排序
    ///
    /// `sorter`: 排序函数
    /// `n`: 测试次数
    pub fn sort<T>(sorter: fn(&mut [T]), n: usize)
    where
        T: Debug + Ord,
        Standard: Distribution<T>,
    {
        // 测试 n 次
        for i in 0..n {
            // 设第 i 次生成的随机数组长度为 i
            let mut array = (0..i)
                .into_iter()
                .map(|_| rand::random())
                .collect::<Vec<_>>();

            // 调用排序函数
            sorter(array.as_mut_slice());

            // 对于排序后数组中的每两个元素，断言前者大于后者
            for pair in array.windows(2) {
                assert!(pair[0] <= pair[1], "{array:?}");
            }
        }
    }

    #[test]
    fn std_sort() {
        // 使用标准库的排序函数，测试测试函数是否正确
        sort::<u8>(|array| array.sort(), 100);
    }

    #[test]
    fn bubble_sort() {
        sort::<u8>(BubbleSort::bubble_sort, 100);
    }

    #[test]
    fn selection_sort() {
        sort::<u8>(SelectionSort::selection_sort, 100);
    }

    #[test]
    fn insertion_sort() {
        sort::<u8>(InsertionSort::insertion_sort, 100);
    }

    #[test]
    fn quick_sort() {
        sort::<u8>(QuickSort::quick_sort, 100);
    }
}
