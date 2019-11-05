#include <iostream>
#include <fstream>

using namespace std;


string replaceFirstOccurrence(
        string& s,
        const string& toReplace,
        const string& replaceWith)
{
    size_t pos = s.find(toReplace);
    if (pos == string::npos) return s;
    return s.replace(pos, toReplace.length(), replaceWith);
}


int main (int argc, char *argv[])
{
    string fileName ;
    string c1       ;
    string c2       ;
    // Default parm
    if ( argc < 3 )
    {
        cout << "Examle command: 4.exe fileName.txt c1 c2\n";

        fileName = "Joe.txt";
        c1 = "this";
        c2 = "sth";

        ofstream myfile;
        myfile.open (fileName.c_str());
        myfile << "Writing this to a this file.\n";
        myfile.close();

    }
    else
    {
        fileName = argv[1];
        string c1 = argv[2];
        string c2 = argv[3];
    }

    // -------------------------------------------------------------

    ifstream ifile;
    ofstream ofile;
    string buff, replaced = "replaced.txt";

    ifile.open (fileName.c_str());
      ofile.open(replaced.c_str());
//    ofile.open (fileName.c_str());

    if( !ifile.is_open() )
        cerr << "Nie udalo sie otworzyć pliku" << endl;

    while( !ifile.eof() )
    {
        getline(ifile, buff);

        while( buff.find(c1) < -1 )
        {
            buff = replaceFirstOccurrence(buff, c1, c2);
            cout << buff;
        }
        ofile <<  buff;
    }

    ifile.close();
    ofile.close();

    remove(fileName.c_str());
    cout << endl << rename( replaced.c_str() , fileName.c_str() );
    remove(replaced.c_str());

    return 0;
}

