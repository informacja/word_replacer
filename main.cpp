#include <iostream>
#include <fstream>

using namespace std;

void replaceAllInFiles(ifstream &ifile, ofstream &ofile, string c1, string c2)
{
    string buff;

    getline (ifile, buff, (char) ifile.eof());

    while( buff.find(c1) != string::npos )                                     // line have more occurrence of to_replace word
    {
        size_t pos = buff.string::find(c1);
        buff.replace(pos, c1.length(), c2);
        cout << "Replaced : " << buff << endl;
    }
//    ofile <<  buff;
}

// --------------------------------------------------------------

int main (int argc, char *argv[])
{
    string fileName, c1, c2;

    // Default parm
    if ( argc < 3 )
    {
        cout << "Example command: 4.exe fileName.txt c1 c2\n";

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
        fileName  = argv[1];
        c1 = argv[2];
        c2 = argv[3];
    }

    // -------------------------------------------------------------

    ifstream ifile, ofile;                                          // input file, output file
                                                    
    string buff, replaced = "replaced.txt";

    ifile.open (fileName.c_str()); ofile.open (replaced.c_str());

    if( !ifile.is_open() )
        cerr << "Input file is not open" << endl;

    replaceAllInFiles( ifile, ofile, c1, c2);

    ifile.close(); ofile.close();

    remove(fileName.c_str());
    cout << endl << rename( replaced.c_str() , fileName.c_str() );
    remove(replaced.c_str());

    return 0;
}

