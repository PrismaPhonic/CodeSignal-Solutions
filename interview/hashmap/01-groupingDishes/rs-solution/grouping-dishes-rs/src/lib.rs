// You are given a list of dishes where each element consists of a list of strings beginning with the name
// of th dish, follow by all the ingredients used to prepare it. You need to group the dishes by ingredients
// so that for each ingredient you'll be able to find all the dishes that contain it.
//
// Return an array where each element is a list beginning with the ingredient name, followed by the names
// of all the dishes that contain this ingredient.
//
// Solution: This seems like a typical hashmap problem. We want to store each ingredient along with a list
// of dishes that the ingredient is in.

use std::collections::HashMap;

fn grouping_dishes(dishes: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut ingredients = HashMap::new();
    for dish_list in &dishes {
        // The dish iteself is always the first item in the list.
        let dish = &dish_list[0];
        // The rest of the items are the ingredients.
        for ingredient in dish_list[1..].into_iter() {
            ingredients.entry(ingredient).or_insert(Vec::new()).push(dish.clone());
        }
    }

    // Convert HashMap back to Vec.
    // Ignore ingredients if they are not in at least two dishes. Sort dishes lexicographically,
    // and outer list lexicographically.
    let mut ingredients_list = Vec::new();
    for (ingredient, dishes) in ingredients {
        if dishes.len() < 2 {
            continue;
        }

        // Sort dishes lexicographically.
        let mut sorted_dishes = dishes.clone();
        sorted_dishes.sort();

        let mut ingredient_list = vec![ingredient.clone()];
        ingredient_list.extend(sorted_dishes);
        ingredients_list.push(ingredient_list);
    }

    ingredients_list.sort_by(|a, b| {
        (&a[0]).partial_cmp(&b[0]).unwrap()
    });
    ingredients_list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn own_strings(input: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        input.into_iter().map(|list| {
            list.into_iter().map(|s| {
                s.to_string()
            }).collect()
        }).collect()
    }

    #[test]
    fn it_works() {
        let input = own_strings(vec![vec!["Salad","Tomato","Cucumber","Salad","Sauce"],
                         vec!["Pizza","Tomato","Sausage","Sauce","Dough"],
                         vec!["Quesadilla","Chicken","Cheese","Sauce"],
                         vec!["Sandwich","Salad","Bread","Tomato","Cheese"]]);
        let want = own_strings(vec![vec!["Cheese","Quesadilla","Sandwich"],
                                    vec!["Salad","Salad","Sandwich"],
                                    vec!["Sauce","Pizza","Quesadilla","Salad"],
                                    vec!["Tomato","Pizza","Salad","Sandwich"]]);
        let got = grouping_dishes(input);
        assert_eq!(got, want);
    }
}
