import re
inputFile = open('/mnt/s/AdventOfCode/2022/input07.txt', 'r')
#inputFile = open('test.txt', 'r')
lines = inputFile.read().split('\n')



def formatInput(lin):
    ret = lin[1:]
    #regex = r''
    #instructions = [[int(a) for a in re.match(regex, x).groups()] for x in lin]
    return ret


formInp = formatInput(lines)
#print(formInp)
folderList = ['/']
fileStructure = {}
for lin in formInp:
    if re.search(r"\$ cd ([a-zA-Z]+)", lin):
        folName = str(re.search(r'\$ cd ([a-zA-Z]+)', lin).group(1))
        folderList.append(folName)
        continue
    if re.search(r'\$ cd ..', lin):
        folderList.pop()
        continue
    if re.search (r'(\d+) .+', lin):
        size = int(re.search(r'(\d+) .+', lin).group(1))
        for idx, a in enumerate(folderList):
            folder = "/".join(folderList[0:idx+1])
            if 'grct' in folderList:
                print(folder, size)
            fileStructure.setdefault(folder, 0)
            fileStructure[folder] += size
#print(fileStructure)
ret = 0
for a in fileStructure:
    print(a + ": " + str(fileStructure[a]))
    if fileStructure[a] <= 100000:
        #print(a + ": " + str(fileStructure[a]))
        ret += fileStructure[a]
print(ret)
