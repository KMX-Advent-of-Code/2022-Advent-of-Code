public static void day7()
    {
        Dictionary<string, Tuple<List<string>, List<Tuple<double, string>>>> directory = new  Dictionary<string, Tuple<List<string>, List<Tuple<double, string>>>> ();
        string currentParent = "";
        string lastLine = "";
        int i = 0;
        List<string> currentSubFolders = new List<string>();
        List<Tuple<double, string>> currentFiles = new List<Tuple<double, string>>();
        string currentFolder = "";

        using(var reader = new StreamReader(@"C:\Users\249916\OneDrive - CarMax\Desktop\Advent Of Code\day 7.txt"))      
        {                       
            while (!reader.EndOfStream)
            {     
                i++; 
                
                string line = reader.ReadLine();      
                var values = line.Split(' ');
                if(values.Count() >= 3 && values[2] == "vmwddb")
                {}
                if(values[0]== "$") // command
                {
                    if(values[1] == "cd")
                    {
                        if(currentSubFolders.Count > 0 || currentFiles.Count > 0)
                        {
                            Tuple<List<string>,List<Tuple<double, string>>> tempCurrentFiles;
                            if(directory.TryGetValue(currentFolder, out tempCurrentFiles))
                            {
                                tempCurrentFiles.Item1.AddRange(currentSubFolders);
                                tempCurrentFiles.Item2.AddRange(currentFiles);
                            }
                            else
                                directory.Add(currentFolder, new Tuple<List<string>, List<Tuple<double, string>>>(currentSubFolders, currentFiles));
                            
                            currentSubFolders = new List<string>();
                            currentFiles = new List<Tuple<double, string>>();
                        
                        }
                        if(values[2] != "..")
                        {
                        if(currentFolder == "")
                            currentFolder = values[2];
                        else
                            currentFolder = currentFolder + "-" + values[2];
                        }                    
                        else if(values[1] == "cd" && values[2] == "..")
                        {
                            var tempFiles = currentFolder.Split('-').ToList();
                            tempFiles.RemoveAt(tempFiles.Count-1);
                            currentFolder = "";
                            for(int j = 0; j<tempFiles.Count; j++ )
                            {
                                if(currentFolder == "")
                                    currentFolder = tempFiles[j];
                                else
                                    currentFolder = currentFolder + "-" + tempFiles[j];
                            }
                        }
                    }            
                }
                else if(values[0] == "dir")
                    currentSubFolders.Add(values[1]);
                else
                    currentFiles.Add(new Tuple<double, string>(Convert.ToDouble(values[0]),values[1]));
             }
        }

        if(currentSubFolders.Count > 0 || currentFiles.Count > 0)
        {
            Tuple<List<string>,List<Tuple<double, string>>> tempCurrentFiles;
            if(directory.TryGetValue(currentFolder, out tempCurrentFiles))
            {
                tempCurrentFiles.Item1.AddRange(currentSubFolders);
                tempCurrentFiles.Item2.AddRange(currentFiles);
            }
            else
                directory.Add(currentFolder, new Tuple<List<string>, List<Tuple<double, string>>>(currentSubFolders, currentFiles));
        
        }

        int countOver = 0;
        double sumOver = 0;
        string drive = "";
        double answer = 99999999999;
        foreach(string key in directory.Keys)
        {
            var result = GetSubValues(key, directory);
            
             if(70000000 - 43956976 + result >= 30000000 && result <= answer)
             {

                    answer = result;
                    drive = key;
                
             }
        }
    }
    public static double GetSubValues(string path, Dictionary<string, Tuple<List<string>, List<Tuple<double, string>>>> directory)
    {
        double size = 0;
        Tuple<List<string>,List<Tuple<double, string>>> tempCurrentFiles;
        if(directory.TryGetValue(path, out tempCurrentFiles))
        {
            size += tempCurrentFiles.Item2.Sum(x => x.Item1);   
            for(int i = 0; i< tempCurrentFiles.Item1.Count; i++ )
            {
                //Console.WriteLine(tempCurrentFiles.Item1[i]);
                //Console.WriteLine(i);
                size += GetSubValues(path + "-" + tempCurrentFiles.Item1[i], directory);                  
            }                       
        }
        return size;
    }
