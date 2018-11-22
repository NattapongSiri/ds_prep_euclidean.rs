# Test data science task with Rust
## Example Business Scenario
Customer want to predict the movement of selected sensors.
## Raw Data
Accumulated positions from 10 sensors that send a position in 3D space (x, y, z) every minutes.
The data format is 
```Python
[[x1,y1,z1], [x2, y2, z2], ...], # sensors data of first minute
[[x1,y1,z1], [x2, y2, z2], ...], # sensors data of second minute
```
An example of 2 records:
```JSON
[[0,0,0], [1,1,1], [2,2,2], [3,3,3], [4,4,4], [5,5,5], [6,6,6], [7,7,7], [8,8,8], [9,9,9]],
[[0,0,0], [2,2,2], [0,0,0], [4,4,4], [5,5,5], [6,6,6], [7,7,7], [8,8,8], [9,9,9], [0, 0, 0]]
```
An data for testing can be found in data.txt
## Data cleansing
Pick first 5 sensors data and leave out all the rest.
For example, the data will become:
```JSON
[[0,0,0], [1,1,1], [2,2,2], [3,3,3], [4,4,4]],
[[0,0,0], [2,2,2], [0,0,0], [4,4,4], [5,5,5]]
```
## Data conversion
Calculate euclidean distance between each record on each sensor.
For example, the result will be:
```JSON
[0, 1, 2, 1, 1]
```