use specification::algorithm::sort::{sort, Sorter};

#[test]
fn bubble_sort_test() {
    {
        use sort::bubble_sort::impl_1::BubbleSorter;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, BubbleSorter, _>(&mut l[..], |&a, &b| a > b);
        BubbleSorter::sort(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::bubble_sort::impl_2::BubbleSorter;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, BubbleSorter, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::bubble_sort::impl_3::BubbleSorter;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, BubbleSorter, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::bubble_sort::impl_4::BubbleSorter;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, BubbleSorter, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
}

#[test]
fn insertion_sort_test() {
    {
        use sort::insertion_sort::impl_1::InsertionSort;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, InsertionSort, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::insertion_sort::impl_2::InsertionSort;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, InsertionSort, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::insertion_sort::impl_3::InsertionSort;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, InsertionSort, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
}

#[test]
fn selection_sort_test() {
    {
        use sort::selection_sort::impl_1::SelectionSort;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, SelectionSort, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::selection_sort::impl_2::SelectionSort;
        let mut l = vec![5, 4, 3, 2, 1];
        sort::<_, SelectionSort, _>(&mut l[..], |&a, &b| a > b);
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
}

#[test]
fn quick_sort_test() {
    {
        use sort::quick_sort::impl_1::QuickSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, QuickSort, _>(&mut l[..], |&a, &b| a > b);
        QuickSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::quick_sort::impl_2::QuickSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, QuickSort, _>(&mut l[..], |&a, &b| a > b);
        QuickSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        // check for overflow
        use sort::quick_sort::impl_2::QuickSort;
        let mut l = vec![0, 4, 3, 2, 1];
        // sort::<_, QuickSort, _>(&mut l[..], |&a, &b| a > b);
        QuickSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![0, 1, 2, 3, 4]];
    }
    {
        // check for overflow
        use sort::quick_sort::impl_3::QuickSort;
        let mut l = vec![0, 4, 3, 2, 1];
        // sort::<_, QuickSort, _>(&mut l[..], |&a, &b| a > b);
        QuickSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![0, 1, 2, 3, 4]];
    }
}

#[test]
fn merge_sort_test() {
    {
        use sort::merge_sort::impl_1::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::merge_sort::impl_2::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::merge_sort::impl_3::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    /* {
        use sort::merge_sort::impl_4::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    */

    {
        use sort::merge_sort::impl_5::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::merge_sort::impl_5::MergeSort;
        let mut l = vec![6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6]];
    }
    {
        use sort::merge_sort::impl_5::MergeSort;
        let mut l = vec![8, 7, 6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6, 7, 8]];
    }
    {
        use sort::merge_sort::impl_5::MergeSort;
        let mut l = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]];
    }

    {
        use sort::merge_sort::impl_6::MergeSort;
        let mut l = vec![5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5]];
    }
    {
        use sort::merge_sort::impl_6::MergeSort;
        let mut l = vec![6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6]];
    }
    {
        use sort::merge_sort::impl_6::MergeSort;
        let mut l = vec![8, 7, 6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6, 7, 8]];
    }
    {
        use sort::merge_sort::impl_6::MergeSort;
        let mut l = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        // sort::<_, MergeSort, _>(&mut l[..], |&a, &b| a > b);
        MergeSort::sort(&mut l[..], &(|&a, &b| a > b));
        assert_eq![l, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]];
    }
}
