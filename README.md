WRITEUP 


How Does Cole Ragans Generate Swing and Misses?

By Aaron Smith

Introduction:

Cole Ragans is an MLB left handed pitcher for the Kansas City Royals. According to baseball reference he stands at 6 '4 weighing 210 pounds. In 2016, the Texas Rangers selected Ragans with the 30th pick in the 1st round; he made his debut for the Rangers August 4, 2022 at the age of 24. Unfortunately, Ragans Major League career started out rough. In 64 and ⅓ innings across 2022 and 2023, Ragans amassed a 5.31 ERA and only struck out 51 batters. As a result, Ragans was dealt to the Kansas City Royals before the 2023 trade deadline. With the Royals, Ragans has reinvented himself. He is striking batters out at a higher clip than ever before and has gained substantial velocity on all of his pitches. This study aims to dive deeper than velocity to determine what has made Ragans so successful as a Royal. Specifically, which of Ragans’ pitches are most successful, and how do pitch type and location affect the outcomes against right-handed and left-handed batters?

Methods & Results:

Data Description: The dataset includes 1798 observations, detailing metrics specific to each pitch. For example each row tracks information like velocity, outcome, launch speed if the ball was put into play, the batter’s handedness, spin axis, movement, and much more vital information pertaining to individual pitches thrown by Ragans. An important note is that the data only contains pitches thrown by Ragans as a Royal from his Kansas City debut to his latest start on April 30th.

I handled the data using two functions I created in rust: read_and_process_data and analyze_data. 

data.rs:

read_and_process_data reads in the csv file and cleans it. It first filters through the data selecting important columns/ variables. The variables selected to analyze were pitch type, release speed, release position, events, descriptions, zone, movement, spin rate, batter handedness, and spin axis.  From there, I dropped any rows who had no entries for pitch type and filled rows without entries for spin axis and events with place holders. I then printed some basic summary statistics on Cole Ragans surrounding pitch usage, velocity and more to get a general idea of his effective pitches. The last thing read_and_process does is filter the processed data frame down to only include rows that involve pitches where there was a swing. This dataframe is then used by the analyze_data(). 

analysis.rs:

Analyze data takes the process data and creates a graph structure. Each node represents a unique combination of batter handedness, pitch_type, and zone. Each edge represents an individual pitch and is weighted based on its effectiveness. Edges are given a value of 1 if the pitch results in a whiff and a 0 for everything else. Additionally, the edges do not connect to other nodes; they are self loops. Lastly, analyze_data calculates the average weight of the edges per node and outputs these scores. The nodes that have the highest average weight for their edges should be regarded as Ragans most effective pitches. The average weight is also the whiff rate due to how I weighted the edges - 1 for swing and miss, and 0 for nothing. 

test.rs:

There is also a test module that ensures  the data processing module correctly processes the csv file into a dataframe and that the analyze data module correctly creates the graph. The test is done on a subsection of the Ragans data set, only 4 pitches, so that I already know how many nodes and edges there should be.

main.rs:

The main module uses both the data and analysis module to produce the output. Which should be the summary statistics for Cole Ragans Pitches along with the whiff rates per node. 

Results:

Reference this figure for details on zone.

https://www.researchgate.net/figure/Game-Day-Zones-as-defined-by-Statcast-and-Baseball-Savant-The-strike-zone-is-presented_fig1_358572353

Cole Ragans most effective pitches in generating swing and misses are as follows. 

Pitch: SL - Batter: L - Zone: 13, Success Rate: 0.71

Pitch: KC - Batter: R - Zone: 13, Success Rate: 0.69

Pitch: CH - Batter: R - Zone: 13, Success Rate: 0.67

Pitch: SL - Batter: R - Zone: 13, Success Rate: 0.61

Pitch: FC - Batter: R - Zone: 13, Success Rate: 0.56

Pitch: CH - Batter: R - Zone: 9, Success Rate: 0.54

Pitch: CH - Batter: R - Zone: 14, Success Rate: 0.52

Pitch: FF - Batter: R - Zone: 11, Success Rate: 0.50

Pitch: SL - Batter: L - Zone: 7, Success Rate: 0.46

Pitch: CH - Batter: R - Zone: 5, Success Rate: 0.38

Pitch: FF - Batter: R - Zone: 6, Success Rate: 0.32

Pitch: FF - Batter: R - Zone: 12, Success Rate: 0.31

Pitch: FF - Batter: R - Zone: 2, Success Rate: 0.31

Pitch: CH - Batter: R - Zone: 8, Success Rate: 0.27

Pitch: FF - Batter: R - Zone: 8, Success Rate: 0.27

Pitch: CH - Batter: R - Zone: 6, Success Rate: 0.25

Pitch: FF - Batter: R - Zone: 1, Success Rate: 0.22

Pitch: FF - Batter: L - Zone: 2, Success Rate: 0.21

Pitch: FC - Batter: R - Zone: 8, Success Rate: 0.20

Pitch: KC - Batter: R - Zone: 8, Success Rate: 0.20

Pitch: FC - Batter: R - Zone: 4, Success Rate: 0.09

Pitch: FF - Batter: R - Zone: 9, Success Rate: 0.09

Pitch: FC - Batter: R - Zone: 1, Success Rate: 0.08

Pitch: FF - Batter: R - Zone: 4, Success Rate: 0.04


The findings demonstrate that Ragans is extremely effective at generating whiffs when locating zone 13. Ragans’ slider is also very effective in generating whiffs to both right and left handed batters. Interestingly enough, Ragans Changeup is extremely effective when thrown to right handed hitters, but is far less effective against left handed hitters. Ragans’ four seam fastball also generates a good amount of whiffs no matter if it is located in the zone or outside of the zone.  Unsurprisingly, pitches outside of what is considered the strike zone generate large amounts of swing and misses when swung at. 

Ultimately, it seems Ragans has found success by employing an arsenal of changups and fastballs to right handed hitters, while featuring his slider more heavily to left handed hitters. In conclusion, Ragans swing and miss pitch, depending on the batter, is either the slider or changeup. 








Cite OpenAI

https://chat.openai.com/share/769d7e6e-07b9-445f-bfc6-8707f7580c95 

https://chat.openai.com/share/ba38ad58-18e8-4826-9eeb-5d627ff4982e 

https://chat.openai.com/share/4edcfc92-0112-48ff-a9da-e6c463ff5abf 

Cite Internet Resources

https://docs.pola.rs/docs/rust/dev/polars/

https://doc.rust-lang.org/std/collections/struct.HashMap.html 

https://docs.rs/petgraph/latest/petgraph/ 


Helped with Rust syntax and errors. I also originally had all my code in one main function. Openai helped me split it into modules. 
