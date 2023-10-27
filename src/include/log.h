
/**
 * @file log.h
 * @brief 日志封装
 * @author Zone.N (Zone.Niuzh@hotmail.com)
 * @version 1.0
 * @date 2023-10-27
 * @copyright MIT LICENSE
 * https://github.com/Simple-XX/SimpleCompiler
 * @par change log:
 * <table>
 * <tr><th>Date<th>Author<th>Description
 * <tr><td>2023-10-27<td>Zone.N<td>创建文件
 * </table>
 */

#ifndef SIMPLECOMPILER_LOG_H
#define SIMPLECOMPILER_LOG_H

#include <sys/time.h>

#include <spdlog/spdlog.h>

#include "config.h"

extern std::shared_ptr<spdlog::logger> SCLOG;

void log_init(void);

#endif /* SIMPLECOMPILER_LOG_H */
