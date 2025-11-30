// APIキー管理のJavaScript

(function () {
  'use strict';

  // DOM要素
  const apiKeysButton = document.getElementById('api-keys-button');
  const apiKeysModal = document.getElementById('api-keys-modal');
  const apiKeysModalClose = document.getElementById('api-keys-modal-close');
  const apiKeysModalOk = document.getElementById('api-keys-modal-ok');
  const apiKeysTbody = document.getElementById('api-keys-tbody');
  const apiKeyNameInput = document.getElementById('api-key-name');
  const apiKeyExpirySelect = document.getElementById('api-key-expiry');
  const createApiKeyButton = document.getElementById('create-api-key');
  const newKeyDisplay = document.getElementById('new-key-display');
  const newKeyValue = document.getElementById('new-key-value');
  const copyApiKeyButton = document.getElementById('copy-api-key');

  let apiKeys = [];

  // モーダルを開く
  function openModal() {
    apiKeysModal.classList.remove('hidden');
    newKeyDisplay.classList.add('hidden');
    loadApiKeys();
  }

  // モーダルを閉じる
  function closeModal() {
    apiKeysModal.classList.add('hidden');
    newKeyDisplay.classList.add('hidden');
    apiKeyNameInput.value = '';
    apiKeyExpirySelect.value = '';
  }

  // APIキー一覧を読み込む
  async function loadApiKeys() {
    try {
      const response = await authenticatedFetch('/api/api-keys');
      if (response.ok) {
        const data = await response.json();
        apiKeys = data.api_keys || data || [];
        renderApiKeys();
      } else if (response.status === 401 || response.status === 403) {
        showError('認証が必要です。ログインしてください。');
      } else {
        showError('APIキーの読み込みに失敗しました');
      }
    } catch (error) {
      console.error('Failed to load API keys:', error);
      showError('APIキーの読み込みに失敗しました');
    }
  }

  // APIキー一覧を表示
  function renderApiKeys() {
    if (!Array.isArray(apiKeys) || apiKeys.length === 0) {
      apiKeysTbody.innerHTML = '<tr><td colspan="4" class="empty-message">APIキーがありません</td></tr>';
      return;
    }

    apiKeysTbody.innerHTML = apiKeys
      .map((key) => {
        const createdAt = new Date(key.created_at).toLocaleString('ja-JP');
        const expiresAt = key.expires_at ? new Date(key.expires_at).toLocaleString('ja-JP') : '無期限';

        return `
          <tr>
            <td>${escapeHtml(key.name)}</td>
            <td>${createdAt}</td>
            <td>${expiresAt}</td>
            <td>
              <button class="btn btn--danger btn--small delete-api-key" data-id="${key.id}">削除</button>
            </td>
          </tr>
        `;
      })
      .join('');

    // 削除ボタンのイベントリスナーを追加
    document.querySelectorAll('.delete-api-key').forEach((btn) => {
      btn.addEventListener('click', function () {
        const keyId = this.dataset.id;
        deleteApiKey(keyId);
      });
    });
  }

  // APIキーを発行
  async function createApiKey() {
    const name = apiKeyNameInput.value.trim();
    const expiryDays = apiKeyExpirySelect.value;

    if (!name) {
      alert('キーの名前を入力してください');
      return;
    }

    let expiresAt = null;
    if (expiryDays) {
      const date = new Date();
      date.setDate(date.getDate() + parseInt(expiryDays, 10));
      expiresAt = date.toISOString();
    }

    try {
      const response = await authenticatedFetch('/api/api-keys', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name,
          expires_at: expiresAt,
        }),
      });

      if (response.ok) {
        const data = await response.json();
        showNewKey(data.key);
        apiKeyNameInput.value = '';
        apiKeyExpirySelect.value = '';
        loadApiKeys();
      } else {
        const error = await response.json().catch(() => ({}));
        alert(error.message || error.error || 'APIキーの発行に失敗しました');
      }
    } catch (error) {
      console.error('Failed to create API key:', error);
      alert('APIキーの発行に失敗しました');
    }
  }

  // 発行されたキーを表示
  function showNewKey(key) {
    newKeyValue.textContent = key;
    newKeyDisplay.classList.remove('hidden');
  }

  // APIキーを削除
  async function deleteApiKey(keyId) {
    if (!confirm('このAPIキーを削除しますか？')) {
      return;
    }

    try {
      const response = await authenticatedFetch(`/api/api-keys/${keyId}`, {
        method: 'DELETE',
      });

      if (response.ok || response.status === 204) {
        loadApiKeys();
      } else {
        alert('APIキーの削除に失敗しました');
      }
    } catch (error) {
      console.error('Failed to delete API key:', error);
      alert('APIキーの削除に失敗しました');
    }
  }

  // クリップボードにコピー
  async function copyToClipboard() {
    const key = newKeyValue.textContent;
    try {
      await navigator.clipboard.writeText(key);
      alert('APIキーをコピーしました');
    } catch (error) {
      // フォールバック
      const textarea = document.createElement('textarea');
      textarea.value = key;
      document.body.appendChild(textarea);
      textarea.select();
      document.execCommand('copy');
      document.body.removeChild(textarea);
      alert('APIキーをコピーしました');
    }
  }

  // エラーメッセージを表示
  function showError(message) {
    apiKeysTbody.innerHTML = `<tr><td colspan="4" class="empty-message" style="color: #c53030;">${escapeHtml(message)}</td></tr>`;
  }

  // HTMLエスケープ
  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // イベントリスナー
  if (apiKeysButton) {
    apiKeysButton.addEventListener('click', openModal);
  }
  if (apiKeysModalClose) {
    apiKeysModalClose.addEventListener('click', closeModal);
  }
  if (apiKeysModalOk) {
    apiKeysModalOk.addEventListener('click', closeModal);
  }
  if (createApiKeyButton) {
    createApiKeyButton.addEventListener('click', createApiKey);
  }
  if (copyApiKeyButton) {
    copyApiKeyButton.addEventListener('click', copyToClipboard);
  }

  // モーダル背景クリックで閉じる
  if (apiKeysModal) {
    apiKeysModal.addEventListener('click', function (e) {
      if (e.target === apiKeysModal) {
        closeModal();
      }
    });
  }

  // ESCキーで閉じる
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape' && apiKeysModal && !apiKeysModal.classList.contains('hidden')) {
      closeModal();
    }
  });
})();
