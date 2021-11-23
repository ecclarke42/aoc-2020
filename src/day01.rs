use itertools::Itertools;

pub const TARGET_SUM: usize = 2020;

// #[cfg(feature = "read_inputs")]
// const INPUTS: &str = include_str!("inputs/01");
// #[cfg(not(feature = "read_inputs"))]
pub const INPUTS: [usize; 200] = [
    1254, 1313, 1456, 1782, 1742, 1391, 1273, 1286, 1373, 1891, 1188, 1994, 1887, 1816, 1984, 961,
    1428, 1105, 1123, 1699, 1154, 1953, 1977, 1450, 1696, 1068, 1241, 1926, 1228, 1591, 1789, 1966,
    1508, 1193, 1190, 1654, 444, 1282, 1169, 1165, 1778, 1669, 1570, 1671, 1845, 1208, 1728, 1798,
    847, 1300, 1817, 1200, 1297, 1658, 1296, 1571, 1991, 1305, 1314, 1903, 1472, 1359, 1506, 1999,
    1877, 1168, 1137, 1288, 1083, 1656, 413, 1430, 1408, 1397, 1846, 1218, 684, 1234, 2007, 900,
    1604, 1460, 1848, 1693, 1324, 1401, 1968, 1918, 1975, 1665, 1413, 1874, 1852, 1521, 1753, 1229,
    1940, 1650, 1976, 1235, 1130, 1927, 1365, 1908, 1441, 1302, 1986, 1449, 1692, 1944, 1277, 1312,
    1826, 1231, 1289, 1814, 1170, 1606, 1608, 1773, 1883, 1936, 1626, 1497, 1860, 1673, 1295, 2005,
    1481, 1995, 1734, 1646, 1557, 1298, 1174, 1894, 1309, 1240, 1982, 1414, 1799, 1620, 1863, 1220,
    1642, 508, 1146, 1187, 1253, 1284, 1952, 1527, 1610, 1333, 1221, 1362, 1457, 1721, 1493, 1330,
    156, 1647, 1841, 1724, 2000, 1398, 1745, 1985, 1269, 1722, 2001, 1923, 1395, 1094, 1140, 1958,
    1239, 1336, 1588, 1338, 1750, 1757, 1444, 1822, 1335, 1941, 1865, 1767, 1881, 1499, 1157, 1990,
    1210, 1779, 1201, 1784, 1961, 1476, 1861, 1468,
];

pub fn find_pair_with_sum_naive(inputs: &[usize], target: usize) -> Option<(usize, usize)> {
    // TODO: or use itertools::tuple_combinations
    let mut skip = 1;
    for x in inputs.iter() {
        for y in inputs.iter().skip(skip) {
            if x + y == target {
                return Some((*x, *y));
            }
        }
        skip += 1;
    }
    None
}

pub fn find_pair_with_sum(inputs: &[usize], target: usize) -> Option<(usize, usize)> {
    inputs.iter().tuple_combinations().find_map(|(a, b)| {
        if a + b == target {
            Some((*a, *b))
        } else {
            None
        }
    })
}

pub fn find_triple_with_sum(inputs: &[usize], target: usize) -> Option<(usize, usize, usize)> {
    inputs.iter().tuple_combinations().find_map(|(a, b, c)| {
        if a + b + c == target {
            Some((*a, *b, *c))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_naive_works() {
        let (x, y) = find_pair_with_sum_naive(&INPUTS, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y, 913824);
    }

    #[test]
    fn pair_iter_works() {
        let (x, y) = find_pair_with_sum(&INPUTS, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y, 913824);
    }

    #[test]
    fn triple_works() {
        let (x, y, z) = find_triple_with_sum(&INPUTS, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y * z, 240889536);
    }
}
