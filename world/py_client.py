import cv2
import mediapipe as mp
import socket
import time

mp_hands = mp.solutions.hands
HOST = '127.0.0.1'
PORT = 8888

# 소켓 생성
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((HOST, PORT))

with mp_hands.Hands(static_image_mode=False, max_num_hands=2, min_detection_confidence=0.5) as hands:
    cap = cv2.VideoCapture(0, cv2.CAP_DSHOW)

    while cap.isOpened():
        success, image = cap.read()
        if not success:
            print("카메라를 찾을 수 없습니다.")
            break
        
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        results = hands.process(image)

        left_hand_landmarks = None
        right_hand_landmarks = None

        if results.multi_hand_landmarks:
            for hand_landmarks in results.multi_hand_landmarks:
                handedness = results.multi_handedness[results.multi_hand_landmarks.index(hand_landmarks)]
                if handedness.classification[0].label == "Left":
                    left_hand_landmarks = [[landmark.x, landmark.y, landmark.z] for landmark in hand_landmarks.landmark]
                elif handedness.classification[0].label == "Right":
                    right_hand_landmarks = [[landmark.x, landmark.y, landmark.z] for landmark in hand_landmarks.landmark]

        # 랜드마크 값을 리스트 형태로 전송
        landmark_list = [left_hand_landmarks, right_hand_landmarks]
        landmark_str = str(landmark_list)
        sock.sendall((landmark_str + '\n').encode('utf-8'))

        time.sleep(0.1)  # CPU 사용을 줄이기 위한 지연

    cap.release()
sock.close()
