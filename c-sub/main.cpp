
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// main.c for MRNIU/SimpleCompiler.

#include "common.h"

ifstream fin;
ofstream fout;

// 源文件
vector<string> src_files;
// 输出文件
string dest_file = "";
// 绝对路径
string abs_path;
// 路径缓存
static char abs_path_buffer[PATH_BUFFER];

// 命令行参数解析相关定义
extern int optind, opterr, optopt;
extern char * optarg;
static const int LEXICAL_OPT = 256;
static struct option long_options[] = {
	{ "help",    no_argument,        NULL,        'h'             },
	{ "version", no_argument,        NULL,        'v'             },
	{ "output",  required_argument,  NULL,        'o'             },
	{ "lexical", optional_argument,  NULL,        LEXICAL_OPT     },
};

int init(int &argc, char * * &argv) {
	// 初始化目录信息
	abs_path = getcwd(abs_path_buffer, 256);
	abs_path += "/";
	int index = 0;
	int c = 0; // 用于接收选项
	while( (c = getopt_long(argc, argv, "hvo:", long_options, &index) ) != EOF) {
		switch(c) {
		    // 显示帮助信息
		    case 'h':
			    cout	<< "c-sub v0.01\nCopyright(C) MRNIU 2020\n"
			                << "命令格式：[源文件[源文件] -o 输出文件 [选项]][-h|-v]\n"
			                << "\t源文件\t\t必须是以.c结尾的文件\n"
			                << "\t-o\t\t指定输出文件\n"
			                << "\t--lexical[指定文件(可选)]\t显示词法分析过程\n"
			                << "\t-h\t\t显示帮助信息\n"
			                << "\t-v\t\t显示版本信息"
			                << endl;
			    break;
		    // 显示版本信息
		    case 'v':
			    cout	<< "c-sub v0.01\nCopyright(C) MRNIU 2020\n"
			                << "简单的 C 语言子集编译器"
			                << endl;
			    break;
		    case 'o':
			    // 寻找源文件，寻找区间 argv[1:-2]
			    for(int i = 1 ; i < argc - 2 ; i++) {
				    // 添加源文件
				    if(strstr(argv[i], ".c") ) {
					    src_files.push_back(abs_path + argv[i]);
				    }
			    }
			    // 设置输出文件
			    dest_file = abs_path + optarg;
			    break;

		    case LEXICAL_OPT:
			    cout	<< "输出词法分析结果，可指定输出到文件\n"
			                << "[--lexical 输出文件]"
			                << endl;
			    break;
		    // 表示选项不支持
		    case '?':
			    cout	<< "unknow option"
			                << endl;
			    break;
		    default:
			    cout	<< "bikbib"
			                << endl;
			    break;
		}
	}
	return 0;
}

int main(int argc, char * * argv) {
	// 初始化
	// 包括与命令行的交互、获取要操作的文件等
	init(argc, argv);
	// 逐个打开文件
	for(auto i:src_files) {
		fin.open(i, ios::in);
		while(fin.is_open() ) {
			char ch = scanner(fin);
			cout << ch;
		}
	}

	return 0;
}
