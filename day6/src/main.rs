use std::collections::HashSet;
struct CustomFormGroup {
    customs_forms: Vec<String>,
}
impl CustomFormGroup {
    fn new(lines: &[&str]) -> CustomFormGroup {
        let mut customs_forms = Vec::<String>::new();
        for line in lines {
            customs_forms.push(String::from(*line));
        }
        CustomFormGroup { customs_forms }
    }

    fn calc_a_any_answer(&self) -> usize {
        let mut unique_keys = HashSet::<char>::new();
        for customs_form in self.customs_forms.iter() {
            let current_hash: HashSet<char> = customs_form.chars().collect();
            let union = unique_keys.union(&current_hash);
            unique_keys = union.into_iter().cloned().collect();
        }
        unique_keys.len()
    }

    fn calc_b_all_answered(&self) -> usize {
        let mut unique_keys: HashSet<char> = self.customs_forms[0].chars().collect();
        for customs_form in self.customs_forms.iter() {
            let current_hash: HashSet<char> = customs_form.chars().collect();
            unique_keys = unique_keys
                .intersection(&current_hash)
                .into_iter()
                .cloned()
                .collect();
        }
        unique_keys.len()
    }
}

fn parse_input_into_groups(input_data: &str) -> Vec<CustomFormGroup> {
    let lines = &input_data.lines().collect::<Vec<&str>>()[..];
    let mut ret = Vec::<CustomFormGroup>::new();
    let mut start = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            let group = CustomFormGroup::new(&lines[start..i]);
            ret.push(group);
            start = i + 1;
        }
    }
    let group = CustomFormGroup::new(&lines[start..]);
    ret.push(group);
    ret
}

fn calculate_day_a_answer(customs_form_groups: &[CustomFormGroup]) -> usize {
    customs_form_groups
        .iter()
        .map(|customs_form_group| customs_form_group.calc_a_any_answer())
        .sum()
}

fn calculate_day_b_answer(customs_form_groups: &[CustomFormGroup]) -> usize {
    customs_form_groups
        .iter()
        .map(|customs_form_group| customs_form_group.calc_b_all_answered())
        .sum()
}

fn main() {
    let input_data = include_str!("../input_data.txt");
    let groups = parse_input_into_groups(input_data);
    let calc_day_a = calculate_day_a_answer(&groups[..]);
    println!("Day a answer: {}", calc_day_a);
    let calc_day_b = calculate_day_b_answer(&groups[..]);
    println!("Day b answer: {}", calc_day_b);
}

#[cfg(test)]
mod test {
    use crate::calculate_day_a_answer;
    use crate::calculate_day_b_answer;
    use crate::parse_input_into_groups;
    #[test]
    fn test_parse_groups() {
        let test_data = include_str!("../test_data.txt");
        let groups = parse_input_into_groups(test_data);
        assert_eq!(groups.len(), 5);
        assert_eq!(groups[0].customs_forms.len(), 1);
        assert_eq!(groups[0].customs_forms[0], "abc");
        assert_eq!(groups[1].customs_forms.len(), 3);
        assert_eq!(groups[1].customs_forms[0], "a");
        assert_eq!(groups[1].customs_forms[1], "b");
        assert_eq!(groups[1].customs_forms[2], "c");
        assert_eq!(groups[2].customs_forms.len(), 2);
        assert_eq!(groups[2].customs_forms[0], "ab");
        assert_eq!(groups[2].customs_forms[1], "ac");
        assert_eq!(groups[3].customs_forms.len(), 4);
        assert_eq!(groups[3].customs_forms[0], "a");
        assert_eq!(groups[3].customs_forms[1], "a");
        assert_eq!(groups[3].customs_forms[2], "a");
        assert_eq!(groups[3].customs_forms[3], "a");
        assert_eq!(groups[4].customs_forms.len(), 1);
        assert_eq!(groups[4].customs_forms[0], "b");
    }

    #[test]
    fn test_calc_day_a_individually() {
        let test_data = include_str!("../test_data.txt");
        let groups = parse_input_into_groups(test_data);
        for (i, answer) in [3, 3, 3, 1, 1].iter().enumerate() {
            assert_eq!(groups[i].calc_a_any_answer(), *answer);
        }
    }

    #[test]
    fn test_calc_day_a_total() {
        let test_data = include_str!("../test_data.txt");
        let groups = parse_input_into_groups(test_data);
        assert_eq!(calculate_day_a_answer(&groups[..]), 11);
    }

    #[test]
    fn test_calc_day_b_individually() {
        let test_data = include_str!("../test_data.txt");
        let groups = parse_input_into_groups(test_data);
        for (i, answer) in [3, 0, 1, 1, 1].iter().enumerate() {
            assert_eq!(groups[i].calc_b_all_answered(), *answer);
        }
    }

    #[test]
    fn test_calc_day_b_total() {
        let test_data = include_str!("../test_data.txt");
        let groups = parse_input_into_groups(test_data);
        assert_eq!(calculate_day_b_answer(&groups[..]), 6);
    }
}
