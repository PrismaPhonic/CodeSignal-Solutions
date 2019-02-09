def draw_rectangle(canvas, rect):
   draw_asterisks(canvas, rect)
   draw_hyphens(canvas, rect)
   draw_vertical_bars(canvas, rect)
   
   return canvas
   
def draw_asterisks(canvas, rect):
   canvas[rect[1]][rect[0]] = '*';
   canvas[rect[3]][rect[0]] = '*';
   
   canvas[rect[1]][rect[2]] = '*';
   canvas[rect[3]][rect[2]] = '*';
   
def draw_hyphens(canvas, rect):
   for y in [rect[1], rect[3]]:
      for x in range(rect[0]+1, rect[2]):
         canvas[y][x] = '-';
         
def draw_vertical_bars(canvas, rect):
   for y in range(rect[1]+1, rect[3]):
      canvas[y][rect[0]] = '|';
      canvas[y][rect[2]] = '|';
