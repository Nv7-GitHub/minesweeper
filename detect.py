import cv2
import numpy as np

BUF = 5 # Buffer area on side of each crop

# Consts
COLS = 18
ROWS = 14

# Screenshot of minesweeper game
img = cv2.imread("screen.png")

# Detect green square
lower = np.array([48, 0, 0])
upper = np.array([52, 200, 255])

# In range, get bounding box of top
imghsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
top = cv2.inRange(imghsv, lower, upper)
cont, _ = cv2.findContours(top, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
cont = max(cont, key=cv2.contourArea)
box = cv2.boundingRect(cont)

# Get size of squares and top-left corner
sqsize = box[2]//COLS # width of box/COLS
start = (box[1] + box[3], box[0]) # (row + height), col

# Cut
imgs = []
for r in range(0, ROWS):
  row = []
  for c in range(0, COLS):
    cop = img.copy()
    row.append(cop[start[0]+(r*sqsize)+BUF : start[0]+((r+1)*sqsize)-BUF, start[1]+(c*sqsize)+BUF:start[1]+((c+1)*sqsize)-BUF]) # img[start_row:end_row, start_col:end_col]
  imgs.append(row)

# Get numbers
nums = [
  (1, (np.array([100, 0, 0]), np.array([110, 255, 255]))),
  (2, (np.array([55, 0, 0]), np.array([65, 255, 255]))),
  (3, (np.array([0, 50, 0]), np.array([5, 255, 255]))),
  (4, (np.array([140, 0, 0]), np.array([150, 255, 255]))),
  (5, (np.array([15, 215, 0]), np.array([25, 255, 255]))),
  (6, (np.array([90, 0, 0]), np.array([100, 255, 255]))),
  (7, (np.array([0, 0, 50]), np.array([0, 0, 70]))),
  #(8, (np.array([10, 0, 0]), np.array([20, 40, 200]))), # Unreliable, very unlikey
  (9, (np.array([40, 100, 0]), np.array([45, 255, 255]))), # Green
  # Anything else is a 0
] # []tuple{id, (lower, upper)}

# Save
board = []
ind = 0
for rcnt, row in enumerate(imgs):
  r = []
  for c, im in enumerate(row):
    im = cv2.cvtColor(im, cv2.COLOR_BGR2HSV)

    val = 0
    cnt = 100
    for det in nums:
      ra = cv2.inRange(im, det[1][0], det[1][1])
      if cv2.countNonZero(ra) > cnt:
        val = det[0]
        cnt = cv2.countNonZero(ra)
    r.append(val)
  board.append(r)


for line in board:
  print(line)