Collecting data from nyaa and offering preview URL for each information which has Bango.

### Environment

Python 3.9 with Flask, BeautifulSoup, requests

Java: JDK11 with Maven

### Component

Python: Collecting data from nyaa order by uploading count;

Java: Offering web frontend and backend; Using HTTP to communicate with python crawler.

##### Extra attention

Update python flask ip/port info, `MGSList.txt`&`memory.txt`(create them first) path info in `GoNyaa-Server/src/main/resources/application.properties` before using.

You could update MGS Bango list in `MGSList.txt` at the same directory above.