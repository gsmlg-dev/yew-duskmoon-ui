// The bridge only transports snapshots/results; parsing runs in the module Worker.
export class WorkerClient {
  constructor(assetsUrl) {
    const base = new URL(assetsUrl.endsWith('/') ? assetsUrl : `${assetsUrl}/`, document.baseURI);
    this.worker = new Worker(new URL('worker.js', base), { type: 'module', name: 'code-engine-syntax' });
    this.pending = new Map();
    this.nextId = 0;
    this.failed = null;
    this.worker.onmessage = ({ data }) => {
      const pending = this.pending.get(data.id);
      if (!pending) return;
      this.pending.delete(data.id);
      if (data.error) {
        pending.reject({ ...data.error, language: pending.language });
      } else {
        pending.resolve(data);
      }
    };
    const fail = (event) => {
      this.failed = { code: 'worker', message: event.message || 'syntax Worker could not load or decode a message' };
      for (const pending of this.pending.values()) pending.reject(this.failed);
      this.pending.clear();
      this.worker.terminate();
    };
    this.worker.onerror = fail;
    this.worker.onmessageerror = fail;
  }

  request(documentId, revision, language, source) {
    if (this.failed) return Promise.reject(this.failed);
    const id = ++this.nextId;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { documentId, language, resolve, reject });
      try {
        this.worker.postMessage({ id, type: 'highlight', documentId, revision, language, source });
      } catch (error) {
        this.pending.delete(id);
        reject({ code: 'worker', message: String(error) });
      }
    });
  }

  close(documentId) {
    for (const [id, pending] of this.pending) {
      if (pending.documentId === documentId) {
        pending.reject({ code: 'closed', message: 'syntax document is closed' });
        this.pending.delete(id);
      }
    }
    if (!this.failed) this.worker.postMessage({ type: 'close', documentId });
  }

  dispose() {
    this.failed = { code: 'closed', message: 'syntax service is closed' };
    for (const pending of this.pending.values()) pending.reject(this.failed);
    this.pending.clear();
    this.worker.onmessage = null;
    this.worker.onerror = null;
    this.worker.onmessageerror = null;
    this.worker.terminate();
  }
}
