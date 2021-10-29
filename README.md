# lootboxes_monte_carlo
A monte-carlo simulation of the number of "lootboxes" to open to get at least one of each possible outcome

I wondered how many lootboxes I needed to open in order to get all possible outcomes at least one (assuming it is possible to get the same one multiple times and all the outcomes have the same probability.

The actual problem I was solving is the following: 
 - A French brand sells food, and in each box there is a magnet representing one small part of the map of the country
 - There are 95 different magnets
 - How many boxes should I expect to buy in order to get the complete map of the country ?

As I was starting to learn rust, I took this as an exercise and wrote this program that generates a video of the resulting curve of the simulations as function of the number of simulations
