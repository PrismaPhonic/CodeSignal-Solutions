function groupingDishes(dishes) {
    // pre-sort dishes array based on first index - this will
    // ensure our arrays for each ingredient are sorted too
    dishes.sort((a, b) => {
        if (a[0] < b[0]) return -1;
        if (a[0] > b[0]) return 1;
        return 0;
    });
    let ingredientDishes = {};
    
    // go through and setup an object where the keys are the 
    // ingredients and values is array of dishes ingredient is in
    for (let dish of dishes) {
        for (let i = 1; i < dish.length; i++) {
            if (!ingredientDishes[dish[i]]) {
                ingredientDishes[dish[i]] = [dish[0]];
            } else {
                ingredientDishes[dish[i]].push(dish[0]);
            }
        }
    }
    
    let output = [];
    for (let [k, v] of Object.entries(ingredientDishes)) {
        if (v.length > 1) {
            output.push([k, ...v]);
        }
    }
    
    // Rearrange lists order so first element of each array is 
    // in alphabetic order respective of all lists
    for (let list of output) {
        output.sort((a, b) => {
            if (a[0] < b[0]) return -1;
            if (a[0] > b[0]) return 1;
            return 0;
        })
    }
    
    return output;
}
