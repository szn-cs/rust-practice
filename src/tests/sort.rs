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
