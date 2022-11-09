function padTo2Digits(num: number|string) {
    return String(num).padStart(2, '0');
}

const getHour = (s: number) => {
    const now = new Date(s*1000);
    
    return padTo2Digits(now.getHours())
    + 'h' + padTo2Digits(now.getMinutes());
};

export { getHour, padTo2Digits };