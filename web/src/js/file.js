export function write_file(name, data) {
    const file = new File(data, name)
    const url = URL.createObjectURL(file)
    const link = document.createElement('a')
    link.href = url
    link.download = name
    link.click()
    URL.revokeObjectURL(url)
}
