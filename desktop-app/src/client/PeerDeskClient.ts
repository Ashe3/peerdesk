export class PeerDeskClient {
  private ws: WebSocket | null = null;
  private screenImage: HTMLImageElement;
  private connectBtn: HTMLButtonElement;
  private disconnectBtn: HTMLButtonElement;
  private serverUrlInput: HTMLInputElement;
  private statusDiv: HTMLDivElement;

  constructor() {
    this.screenImage = document.getElementById(
      'screenImage'
    ) as HTMLImageElement;
    this.connectBtn = document.getElementById(
      'connectBtn'
    ) as HTMLButtonElement;
    this.disconnectBtn = document.getElementById(
      'disconnectBtn'
    ) as HTMLButtonElement;
    this.serverUrlInput = document.getElementById(
      'serverUrl'
    ) as HTMLInputElement;
    this.statusDiv = document.getElementById('status') as HTMLDivElement;

    this.connectBtn.addEventListener('click', () => this.connect());
    this.disconnectBtn.addEventListener('click', () => this.disconnect());
  }

  connect() {
    const url = this.serverUrlInput.value;
    this.updateStatus('Connecting...');

    this.ws = new WebSocket(url);
    this.ws.binaryType = 'arraybuffer';

    this.ws.onopen = () => {
      this.updateStatus('Connected');
      this.connectBtn.disabled = true;
      this.disconnectBtn.disabled = false;
    };

    this.ws.onmessage = (event) => {
      // Получаем JPEG-байты и отображаем как изображение
      const blob = new Blob([event.data], { type: 'image/jpeg' });
      const url = URL.createObjectURL(blob);
      this.screenImage.src = url;

      // Освобождаем URL после загрузки
      this.screenImage.onload = () => URL.revokeObjectURL(url);
    };

    this.ws.onclose = () => {
      this.updateStatus('Disconnected');
      this.connectBtn.disabled = false;
      this.disconnectBtn.disabled = true;
    };

    this.ws.onerror = (error) => {
      this.updateStatus('Connection error');
      console.error('WebSocket error:', error);
    };
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
    }
  }

  updateStatus(status: string) {
    this.statusDiv.textContent = status;
  }
}
