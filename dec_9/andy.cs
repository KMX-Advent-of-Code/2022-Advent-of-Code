public static void day9()
    {
        List<List<Tuple<int, int>>> location = new List<List<Tuple<int, int>>>();


        using(var reader = new StreamReader(@"C:\Users\249916\OneDrive - CarMax\Desktop\Advent Of Code\day 9.txt"))
        {
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            location.Add(new List<Tuple<int, int>>(){new Tuple<int,int>(0,0) });
            while (!reader.EndOfStream)
            {
                var line = reader.ReadLine();
                var values = line.Split(' ');
                var distance = Convert.ToInt32(values[1]);
                string direction = values[0];            
                var headLocations = location[0];
                for(int i = 1; i<= distance; i++)
                {
                    var currentHeadLoc = location[0].Last();
                    if(direction == "R")
                    {                    
                        location[0].Add(new Tuple<int,int>(currentHeadLoc.Item1 + 1, currentHeadLoc.Item2));                        
                    }
                    else if(direction == "L")
                    {                         
                        location[0].Add(new Tuple<int,int>(currentHeadLoc.Item1 -1, currentHeadLoc.Item2));                 
                    }
                    else if(direction == "U")
                    {
                        location[0].Add(new Tuple<int,int>(currentHeadLoc.Item1, currentHeadLoc.Item2 + 1));                        
                    }
                    else if(direction == "D")
                    {
                        location[0].Add(new Tuple<int,int>(currentHeadLoc.Item1, currentHeadLoc.Item2 - 1));                     
                    }
                    for(int j = 1; j<=9; j++)
                    {
                        var newY = move(location[j-1].Last(),location[j].Last());
                        if(newY != null)
                            location[j].Add(newY);
                    }
                }               
            }
        }
        location[9].Distinct().Count();
    }
    public static Tuple<int,int> move (Tuple<int, int> currentHeadLoc, Tuple<int, int>currentTailLoc)
    {
        if(currentHeadLoc.Item1 == currentTailLoc.Item1)// Same X
        {                  
            if(currentHeadLoc.Item2 == currentTailLoc.Item2)
                return null; //Same spot
            else if(currentHeadLoc.Item2 > currentTailLoc.Item2 + 1)
                return new Tuple<int,int>(currentTailLoc.Item1, currentTailLoc.Item2 + 1); //Move up
            else if(currentHeadLoc.Item2 < currentTailLoc.Item2 - 1)
                return new Tuple<int,int>(currentTailLoc.Item1, currentTailLoc.Item2 - 1); //Move down
        }
        else if (currentHeadLoc.Item2 == currentTailLoc.Item2)// Same Y
        {                  
             if(currentHeadLoc.Item1 > currentTailLoc.Item1 + 1)
                return new Tuple<int,int>(currentTailLoc.Item1 + 1, currentTailLoc.Item2); //Move right
            else if(currentHeadLoc.Item1 < currentTailLoc.Item1 - 1)
                return new Tuple<int,int>(currentTailLoc.Item1 - 1, currentTailLoc.Item2); //Move left
        }
        else 
        {
            if(currentHeadLoc.Item2 >  currentTailLoc.Item2 + 1)  // above
            {
                if(currentHeadLoc.Item1 >  currentTailLoc.Item1) // To the Right
                {
                    return new Tuple<int,int>(currentTailLoc.Item1 + 1, currentTailLoc.Item2 + 1); //Move left
                }
                else 
                {                    
                    return new Tuple<int,int>(currentTailLoc.Item1 - 1, currentTailLoc.Item2 + 1); //Move left
                }
            } 
            else if(currentHeadLoc.Item2 <  currentTailLoc.Item2 - 1)  // below
            {
                if(currentHeadLoc.Item1 >  currentTailLoc.Item1) // To the Right
                {
                    return new Tuple<int,int>(currentTailLoc.Item1 + 1, currentTailLoc.Item2 - 1); //Move left
                }
                else
                {                    
                    return new Tuple<int,int>(currentTailLoc.Item1 - 1, currentTailLoc.Item2 - 1); //Move left
                }
            } 
            else if(currentHeadLoc.Item1 >  currentTailLoc.Item1 + 1)  // right
            {
                if(currentHeadLoc.Item2 >  currentTailLoc.Item2) // To the above
                {
                    return new Tuple<int,int>(currentTailLoc.Item1 + 1, currentTailLoc.Item2 + 1); //Move left
                }
                else 
                {                    
                    return new Tuple<int,int>(currentTailLoc.Item1 + 1, currentTailLoc.Item2 - 1); //Move left
                }
            } 
            else if(currentHeadLoc.Item1 <  currentTailLoc.Item1 - 1)  // below
            {
                if(currentHeadLoc.Item2 >  currentTailLoc.Item2) // To the Right
                {
                    return new Tuple<int,int>(currentTailLoc.Item1 - 1, currentTailLoc.Item2 + 1); //Move left
                }
                else
                {                    
                    return new Tuple<int,int>(currentTailLoc.Item1 - 1, currentTailLoc.Item2 - 1); //Move left
                }
            } 
        }
        return null;
      // var results = new Tuple<List<Tuple<int, int>>, List<Tuple<int, int>>>();
    }
