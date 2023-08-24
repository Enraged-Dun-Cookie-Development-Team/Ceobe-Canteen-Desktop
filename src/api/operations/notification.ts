class NotificationOperate {
  getInfo<T>(callback: (event: string, payload: T) => void) {}

  closeWindow() {}
}
const notification = new NotificationOperate();

export default notification;
