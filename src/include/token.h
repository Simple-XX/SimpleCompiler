
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
enum tag_t : ssize_t {
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
  tag_t tag;

  /**
   * 构造函数
   * @param _tag tag
   */
  explicit token_base_t(tag_t _tag);

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
 * 标识符(变量名)
 */
class token_identifier_t : public token_base_t {
public:
  /// 标识符名称
  std::string name;

  /**
   * 构造函数
   * @param _name 标识符名称
   */
  explicit token_identifier_t(std::string _name);

  /// @name 默认构造/析构函数
  /// @{
  token_identifier_t() = default;
  token_identifier_t(const token_identifier_t &_token_identifier) = default;
  token_identifier_t(token_identifier_t &&_token_identifier) = default;
  auto operator=(const token_identifier_t &_token_identifier)
      -> token_identifier_t & = default;
  auto operator=(token_identifier_t &&_token_identifier)
      -> token_identifier_t & = default;
  ~token_identifier_t() = default;
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
  /// 数值
  int num_val;

  /**
   * 构造函数
   * @param _num_val 数值
   */
  explicit token_num_t(int _num_val);

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
  /// 字符值
  char char_val;

  /**
   * 构造函数
   * @param _char_val 字符值
   */
  explicit token_char_t(char _char_val);

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
  /// 字符串值
  std::string string_val;

  /**
   * 构造函数
   * @param _string_val 字符串值
   */
  explicit token_string_t(const std::string &_string_val);

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
  /**
   * 构造函数
   */
  keywords_t();

  /// @name 默认构造/析构函数
  /// @{
  keywords_t(const keywords_t &_keywords) = default;
  keywords_t(keywords_t &&_keywords) = default;
  auto operator=(const keywords_t &_keywords) -> keywords_t & = default;
  auto operator=(keywords_t &&_keywords) -> keywords_t & = default;
  ~keywords_t() = default;
  /// @}

  /**
   * 通过字符串获取 tag
   * @param _name 字符串
   * @return tag_t tag 名
   */
  tag_t get_tag(const std::string &_name);

private:
  /// 关键字-tag 表
  std::unordered_map<std::string, tag_t, std::hash<std::string>> keywords;
};

#endif /* SIMPLECOMPILER_TOKEN_H */
