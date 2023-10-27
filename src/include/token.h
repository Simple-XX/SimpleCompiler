
/**
 * @file token.h
 * @brief token
 * @author Zone.N (Zone.Niuzh@hotmail.com)
 * @version 1.0
 * @date 2023-10-26
 * @copyright MIT LICENSE
 * https://github.com/Simple-XX/SimpleCompiler
 * @par change log:
 * <table>
 * <tr><th>Date<th>Author<th>Description
 * <tr><td>2023-10-26<td>Zone.N<td>迁移到 doxygen
 * </table>
 */

#ifndef SIMPLECOMPILER_TOKEN_H
#define SIMPLECOMPILER_TOKEN_H

#include <cstdint>
#include <string>
#include <unordered_map>

/**
 * 支持的类型枚举
 */
enum Tag : ssize_t {
  // 错误
  ERR = -2,
  // 结束标识
  END = EOF,

  /// @name 关键字 keyword
  /// @{
  /// int
  KW_INT = 0,
  /// char
  KW_CHAR,
  /// void
  KW_VOID,
  /// const
  KW_CONST,
  /// if
  KW_IF,
  /// else
  KW_ELSE,
  /// while
  KW_WHILE,
  /// for
  KW_FOR,
  /// break
  KW_BREAK,
  /// continue
  KW_CONTINUE,
  /// return
  KW_RETURN,
  /// @}

  /// @name 类型标识
  /// 标识符
  ID,
  /// int
  NUM,
  /// char
  CHAR,
  /// str
  STR,
  /// @}

  /// @name 操作符 operator
  /// =
  ASSIGN,
  /// +
  ADD,
  /// -
  SUB,
  /// *
  MUL,
  /// /
  DIV,
  /// %
  MOD,
  /// |
  ORBIT,
  /// &
  ANDBIT,
  /// ^
  EORBIT,
  /// &&
  AND,
  /// ||
  OR,
  /// !
  NOT,
  /// >
  GT,
  /// >=
  GE,
  /// <
  LT,
  /// <=
  LE,
  /// ==
  EQU,
  /// !=
  NEQU,
  /// @}

  /// @name 分界符 separator
  /// (
  LPAREN,
  /// )
  RPAREN,
  /// {
  LBRACE,
  /// }
  RBRACE,
  /// [
  LBRACKET,
  /// ]
  RBRACKET,
  /// ,
  COMMA,
  /// :
  COLON,
  /// ;
  SEMICON,
  /// @}
};

/**
 * 基类
 */
class Token {
public:
  /// 保存的 tag
  Tag tag;

  /**
   * 构造函数
   * @param _tag tag
   */
  explicit Token(Tag _tag);

  /// @name 默认构造/析构函数
  /// @{
  Token() = default;
  Token(const Token &_token) = default;
  Token(Token &&_token) = default;
  auto operator=(const Token &_token) -> Token & = default;
  auto operator=(Token &&_token) -> Token & = default;
  virtual ~Token() = default;
  /// @}

  /**
   * 转换为 uint32_t
   * @return uint32_t         结果
   * @return const std::string    转换为字符串结果
   */
  [[nodiscard]] virtual auto to_string() -> const std::string;
};

/**
 * 标识符
 */
class Id : public Token {
public:
  std::string name;

  explicit Id(std::string _n);

  /// @name 默认构造/析构函数
  /// @{
  Id() = default;
  Id(const Id &_id) = default;
  Id(Id &&_id) = default;
  auto operator=(const Id &_id) -> Id & = default;
  auto operator=(Id &&_id) -> Id & = default;
  ~Id() = default;
  /// @}

  [[nodiscard]] auto to_string() -> const std::string override;
};

/**
 * 数字
 */
class Num : public Token {
public:
  int val;

  explicit Num(int _val);

  /// @name 默认构造/析构函数
  /// @{
  Num() = default;
  Num(const Num &_num) = default;
  Num(Num &&_num) = default;
  auto operator=(const Num &_num) -> Num & = default;
  auto operator=(Num &&_num) -> Num & = default;
  ~Num() = default;
  /// @}

  [[nodiscard]] auto to_string() -> const std::string override;
};

/**
 * 字符
 */
class Char : public Token {
public:
  char ch;

  explicit Char(char _ch);

  /// @name 默认构造/析构函数
  /// @{
  Char() = default;
  Char(const Char &_char) = default;
  Char(Char &&_char) = default;
  auto operator=(const Char &_char) -> Char & = default;
  auto operator=(Char &&_char) -> Char & = default;
  ~Char() = default;
  /// @}

  [[nodiscard]] auto to_string() -> const std::string override;
};

/**
 * 字符串
 */
class Str : public Token {
public:
  std::string str;

  explicit Str(const std::string &_string);

  /// @name 默认构造/析构函数
  /// @{
  Str() = default;
  Str(const Str &_str) = default;
  Str(Str &&_str) = default;
  auto operator=(const Str &_str) -> Str & = default;
  auto operator=(Str &&_str) -> Str & = default;
  ~Str() = default;
  /// @}

  [[nodiscard]] auto to_string() -> const std::string override;
};

/**
 * 关键字
 */
class Keywords {
public:
  Keywords();

  /// @name 默认构造/析构函数
  /// @{
  Keywords(const Keywords &_keywords) = default;
  Keywords(Keywords &&_keywords) = default;
  auto operator=(const Keywords &_keywords) -> Keywords & = default;
  auto operator=(Keywords &&_keywords) -> Keywords & = default;
  ~Keywords() = default;
  /// @}

  Tag get_tag(std::string _name);

private:
  std::unordered_map<std::string, Tag, std::hash<std::string>> keywords;
};

#endif /* SIMPLECOMPILER_TOKEN_H */
