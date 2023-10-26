
/**
 * @file buffer_base_test.cpp
 * @brief buffer_base.hpp 测试
 * @author Zone.N (Zone.Niuzh@hotmail.com)
 * @version 1.0
 * @date 2022-09-03
 * @copyright MIT LICENSE
 * https://github.com/Simple-XX/SimpleCompiler
 * @par change log:
 * <table>
 * <tr><th>Date<th>Author<th>Description
 * <tr><td>2022-09-03<td>Zone.N<td>创建文件
 * </table>
 */

#include "gtest/gtest.h"

#include "buffer_base.hpp"

TEST(buffer_base_t, 空构造) {
  SimpleCompiler::buffer_base_t<float> buffer_base0;
  EXPECT_EQ(buffer_base0.get_width(), 0);
  EXPECT_EQ(buffer_base0.get_height(), 0);
  EXPECT_EQ(buffer_base0.length(), 0);
  EXPECT_EQ(buffer_base0.data(), nullptr);
}

TEST(buffer_base_t, 拷贝构造) {
  SimpleCompiler::buffer_base_t<float> buffer_base0((uint32_t)100, 200, 0);
  EXPECT_EQ(buffer_base0.get_width(), 100);
  EXPECT_EQ(buffer_base0.get_height(), 200);
  EXPECT_EQ(buffer_base0.BPP, sizeof(uint32_t));
  EXPECT_EQ(buffer_base0.length(), 100 * sizeof(uint32_t) * 200);
  EXPECT_NE(buffer_base0.data(), nullptr);

  SimpleCompiler::buffer_base_t<float> buffer_base1;
  buffer_base1 = buffer_base0;
  EXPECT_EQ(buffer_base1.get_width(), 100);
  EXPECT_EQ(buffer_base1.get_height(), 200);
  EXPECT_EQ(buffer_base1.BPP, sizeof(uint32_t));
  EXPECT_EQ(buffer_base1.length(), 100 * sizeof(uint32_t) * 200);
  EXPECT_NE(buffer_base1.data(), nullptr);
}

TEST(buffer_base_t, clear) {
  SimpleCompiler::buffer_base_t<float> buffer_base0((uint32_t)100, 200, 0);
  EXPECT_EQ(buffer_base0.get_width(), 100);
  EXPECT_EQ(buffer_base0.get_height(), 200);
  EXPECT_EQ(buffer_base0.BPP, sizeof(uint32_t));
  EXPECT_EQ(buffer_base0.length(), 100 * sizeof(uint32_t) * 200);
  EXPECT_NE(buffer_base0.data(), nullptr);
  EXPECT_EQ(buffer_base0.data()[0], 0);
  buffer_base0.clear(std::numeric_limits<float>::lowest());
  EXPECT_EQ(buffer_base0.data()[0], std::numeric_limits<float>::lowest());
}

TEST(buffer_base_t, 不同大小的buffer赋值) {
  SimpleCompiler::buffer_base_t<float> buffer_base;
  EXPECT_TRUE(buffer_base.get_height() == 0);
  EXPECT_TRUE(buffer_base.get_width() == 0);
}
