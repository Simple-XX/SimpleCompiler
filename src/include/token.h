
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
class token_base_t {
public:
  /// 保存的 tag
  Tag tag;

  /**
   * 构造函数
   * @param _tag tag
   */
  explicit token_base_t(Tag _tag);

  /// @name 默认构造/析构函数
  /// @{
  token_base_t() = default;
  token_base_t(const token_base_t &_token) = default;
  token_base_t(token_base_t &&_token) = default;
  auto operator=(const token_base_t &_token) -> token_base_t & = default;
  auto operator=(token_base_t &&_token) -> token_base_t & = default;
  virtual ~token_base_t() = default;
  /// @}

  /**
   * 获得当前 token 的名称字符串
   * @return const std::string    tag 名
   */
  [[nodiscard]] virtual auto to_string() const -> const std::string;
};

/**
 * 标识符
 */
class Id : public token_base_t {
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

  /**
   * 获得当前 token 的名称字符串
   * @return const std::string    tag 名+标识符名称
   */
  [[nodiscard]] auto to_string() const -> const std::string override;
};

/**
 * 数字
 */
class token_num_t : public token_base_t {
public:
  int val;

  explicit token_num_t(int _val);

  /// @name 默认构造/析构函数
  /// @{
  token_num_t() = default;
  token_num_t(const token_num_t &_num) = default;
  token_num_t(token_num_t &&_num) = default;
  auto operator=(const token_num_t &_num) -> token_num_t & = default;
  auto operator=(token_num_t &&_num) -> token_num_t & = default;
  ~token_num_t() = default;
  /// @}

  /**
   * 获得当前 token 的名称字符串
   * @return const std::string    tag 名+数值
   */
  [[nodiscard]] auto to_string() const -> const std::string override;
};

/**
 * 字符
 */
class token_char_t : public token_base_t {
public:
  char ch;

  explicit token_char_t(char _ch);

  /// @name 默认构造/析构函数
  /// @{
  token_char_t() = default;
  token_char_t(const token_char_t &_char) = default;
  token_char_t(token_char_t &&_char) = default;
  auto operator=(const token_char_t &_char) -> token_char_t & = default;
  auto operator=(token_char_t &&_char) -> token_char_t & = default;
  ~token_char_t() = default;
  /// @}

  /**
   * 获得当前 token 的名称字符串
   * @return const std::string    tag 名+字符
   */
  [[nodiscard]] auto to_string() const -> const std::string override;
};

/**
 * 字符串
 */
class token_string_t : public token_base_t {
public:
  std::string str;

  explicit token_string_t(const std::string &_string);

  /// @name 默认构造/析构函数
  /// @{
  token_string_t() = default;
  token_string_t(const token_string_t &_str) = default;
  token_string_t(token_string_t &&_str) = default;
  auto operator=(const token_string_t &_str) -> token_string_t & = default;
  auto operator=(token_string_t &&_str) -> token_string_t & = default;
  ~token_string_t() = default;
  /// @}

  /**
   * 获得当前 token 的名称字符串
   * @return const std::string    tag 名+字符串
   */
  [[nodiscard]] auto to_string() const -> const std::string override;
};

/**
 * 关键字
 */
class keywords_t {
public:
  keywords_t();

  /// @name 默认构造/析构函数
  /// @{
  keywords_t(const keywords_t &_keywords) = default;
  keywords_t(keywords_t &&_keywords) = default;
  auto operator=(const keywords_t &_keywords) -> keywords_t & = default;
  auto operator=(keywords_t &&_keywords) -> keywords_t & = default;
  ~keywords_t() = default;
  /// @}

  Tag get_tag(std::string _name);

private:
  std::unordered_map<std::string, Tag, std::hash<std::string>> keywords;
};

#endif /* SIMPLECOMPILER_TOKEN_H */
