/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-16 16:45:13
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-16 16:45:53
 * @Description: utils
 * @FilePath: /power-copy/src/utils.ts
 */

/**
 * 日期格式化
 * @param {string|number|undefined|null} date 可转化为Date的字符串或时间戳, 为空时返回当前时间
 * @param {string} fmt 格式化字符串，默认为 yyyy-MM-dd HH:mm:ss
 * @returns {string} 格式化后的日期字符串
 */
export function dateFormat(date: string | number | undefined | null, fmt = 'yyyy-MM-dd HH:mm:ss'): string {
  const dateObj = date ? new Date(date) : new Date();
  const opt = {
    'y+': dateObj.getFullYear().toString(), // 年
    'M+': (dateObj.getMonth() + 1).toString(), // 月
    'd+': dateObj.getDate().toString(), // 日
    'H+': dateObj.getHours().toString(), // 时
    'm+': dateObj.getMinutes().toString(), // 分
    's+': dateObj.getSeconds().toString(), // 秒
  };

  for (const k in opt) {
    const ret = new RegExp(`(${k})`).exec(fmt);
    if (ret) {
      fmt = fmt.replace(
        ret[1],
        ret[1].length == 1 ? opt[k as keyof typeof opt] : opt[k as keyof typeof opt].padStart(ret[1].length, '0')
      );
    }
  }
  return fmt;
}
