import BN from 'bn.js'

const trimTrailingZeroes = (value: string) => {
  return value.replace(/\.?0*$/, '')
}

const formatWithCommas = (value: string) => {
  const pattern = /(-?\d+)(\d{3})/
  while (pattern.test(value)) {
    value = value.replace(pattern, '$1,$2')
  }
  return value
}

export const formatTokenAmount = (
  balance: string,
  decimals: number,
  fracDigits?: number
) => {
  const balanceBN = new BN(balance)
  if (fracDigits && fracDigits !== decimals) {
    const roundingExp = decimals - fracDigits - 1
    if (roundingExp > 0) {
      balanceBN.add(new BN(5).mul(new BN(10).pow(new BN(roundingExp))))
    }
  }
  balance = balanceBN.toString()
  const wholeStr = balance.substring(0, balance.length - decimals) || '0'
  const fractionStr = balance
    .substring(balance.length - decimals)
    .padStart(decimals, '0')
    .substring(0, fracDigits)
  return trimTrailingZeroes(`${formatWithCommas(wholeStr)}.${fractionStr}`)
}
